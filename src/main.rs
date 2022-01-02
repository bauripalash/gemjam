use gemjam::jam::gemtext::scanner::GemScanner;
use gemjam::jam::gemtext::transformer::GemTransformer;

fn main() {
    let _x_ = "
    # header_one world are you this is very fun to work with
    #### this is a very good job we have done
        hello
    i think it is a mango
    ## header_two
    ### header_three
    * list_one
    * list_two


    ```rust
    hello world
    mew
    mew
    pew
    ``` 
    => https://palashbauri.in hello
    => https://google.com
    > this is quote
    ";
    let s = String::from(_x_);
    let mut gs = GemScanner::new(s);
    gs.scan_tokens();
    gs.print_tokens();
    let mut t = GemTransformer::new(gs.get_tokens());
    t.transform();
}
