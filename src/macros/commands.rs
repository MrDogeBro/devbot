#[macro_export]
macro_rules! register_commands {
    ( $fwk: expr, $cmd: expr, $cat: tt ) => ($fwk.command($cmd(), |f| f.category($cat)));
    ( $fwk: expr, $mdl: path, $cat: tt, $( $cmd: tt ),+ ) => {
        {
            use $mdl as module;
            $(
                $fwk.command(module::$cmd(), |f| f.category($cat));
            )+
        }
    };
}
