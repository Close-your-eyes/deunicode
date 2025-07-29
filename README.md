
<!-- README.md is generated from README.Rmd. Please edit that file -->

# deunicode

<!-- badges: start -->

<!-- badges: end -->

This is a wrapper around the rust crate deunicode:
<https://docs.rs/deunicode/latest/deunicode/>

## Installation

Requires rust compiler: <https://www.rust-lang.org/>

Install on mac or linux with: curl –proto ‘=https’ –tlsv1.2 -sSf
<https://sh.rustup.rs> \| sh

``` r
# then in R:
devtools::install_github("Close-your-eyes/deunicode")
```

## Example

``` r
# library(deunicode)
deunicode::deunicode(x = c("näme1",
                           "næme2",
                           "nameç",
                           "北亰",
                           "げんまい茶",
                           "🦄☣",
                           "…",
                           "ᔕᓇᓇ"))
#> [1] "name1"             "naeme2"            "namec"             "Bei Jing"          "genmaiCha"         "unicorn biohazard" "..."              
#> [8] "shanana"
```
