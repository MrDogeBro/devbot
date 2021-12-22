#[macro_export]
macro_rules! register_commands {
    ( $fwk: expr, $cmd: expr) => ($fwk.command($cmd(), |f| f));
    ( $fwk: expr, $mdl: path, $( $cmd: tt ),+ ) => {
        {
            use $mdl as module;
            $(
                $fwk.command(module::$cmd(), |f| f);
            )+
        }
    };
}

#[macro_export]
macro_rules! register_commands_group {
    ( $fwk: expr, $cmd: expr) => ($fwk.command($cmd(), |f| f));
    ( $fwk: expr, $mdl: path, $base_cmd: tt, $( $cmd: tt ),+ ) => {
        {
            use $mdl as module;
            $fwk.command(module::$base_cmd(), |f| {
                $(
                    f.subcommand(module::$cmd(), |f| f);
                )+
                f
            });
        }
    };
}
