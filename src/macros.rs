macro_rules! repeat_over_modules {
    ($function:ident in $( $module:path ),+) => {
        {
            let mut results = Vec::new();
    
            $(
                {
                    use $module::{$function};
                    results.push($function());
                }
            )+
    
            results
        }
    };
}