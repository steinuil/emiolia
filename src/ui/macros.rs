macro_rules! actions {
    ($app:expr, $widget:expr, $sender:expr, $group:ident, {
        $($action:ident$([$title:expr, $accel:expr])? => $msg:expr),*$(,)?
    }) => {
        {
            let mut actions = ::relm4::actions::RelmActionGroup::<$group>::new();

            $(
                actions.add_action(::relm4::actions::RelmAction::<$action>::new_stateless({
                    let sender = $sender.clone();
                    move |_| {
                        sender.input($msg);
                    }
                }));
                $($app.set_accels_for_action(&<$action as ::relm4::actions::ActionName>::action_name(), &[$accel]);)?
            )*

            actions.register_for_widget($widget);

            let mut shortcuts = vec![];

            $(
                $(
                    shortcuts.push(($title.to_string(), $accel.to_string()));
                )?
            )*

            shortcuts
        }
    };
}

pub(crate) use actions;

macro_rules! unwrap_or_return {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(_) => return,
        }
    };
}

pub(crate) use unwrap_or_return;
