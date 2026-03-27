use soup::prelude::SessionExt as _;

pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

pub struct TransmissionClient {
    uri: glib::Uri,
    auth: Option<LoginInfo>,
    session_id: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request failed: {0}")]
    RequestFailed(glib::Error),

    #[error("request failed with status={0:?}")]
    BadStatus(soup::Status, Vec<u8>),

    #[error("transmission returned a 409 response without a X-Transmission-Session-Id header")]
    NoSessionId,
}

const SESSION_ID_HEADER: &str = "X-Transmission-Session-Id";

impl TransmissionClient {
    async fn send_request(
        &self,
        session: &soup::Session,
    ) -> Result<(soup::Message, glib::Bytes), Error> {
        let message = soup::Message::from_uri("POST", &self.uri);
        message.set_request_body_from_bytes(
            Some("application/json"),
            Some(&glib::Bytes::from_owned("")),
        );
        if let Some(session_id) = &self.session_id {
            message
                .request_headers()
                .expect("request headers")
                .append(SESSION_ID_HEADER, session_id);
        }

        // TODO add authentication

        let response = session
            .send_and_read_future(&message, glib::Priority::DEFAULT_IDLE)
            .await
            .map_err(Error::RequestFailed)?;

        Ok((message, response))
    }

    async fn request(&mut self, session: &soup::Session) -> Result<Vec<u8>, Error> {
        let (message, response) = self.send_request(session).await?;

        let (message, response) = if message.status() == soup::Status::Conflict {
            self.session_id = Some(
                message
                    .response_headers()
                    .ok_or(Error::NoSessionId)?
                    .one(SESSION_ID_HEADER)
                    .ok_or(Error::NoSessionId)?
                    .to_string(),
            );
            self.send_request(session).await?
        } else {
            (message, response)
        };

        let response = response.to_vec();

        match message.status() {
            soup::Status::Ok => Ok(response),
            status => Err(Error::BadStatus(status, response)),
        }
    }
}
