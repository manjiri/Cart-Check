use shopify_function::prelude::*;
use shopify_function::Result;
use crate::input::InputCartLinesMerchandise::ProductVariant;
use crate::input::InputCartLinesMerchandiseOnProductVariant;
use serde::{ Serialize};
use serde_json::{Value};

// Use the shopify_function crate to generate structs for the function input and output
generate_types!(
    query_path = "./input.graphql",
    schema_path = "./schema.graphql"
);

// Use the shopify_function crate to declare your function entrypoint
#[shopify_function]
fn function(input: input::ResponseData) -> Result<output::FunctionResult> {

    let cart_lines = input.cart.lines;

    if cart_lines.is_empty()  {
        return Ok(output::FunctionResult {
            discount_application_strategy: output::DiscountApplicationStrategy::FIRST,
            discounts: vec![],
        });
    }
    let mut discounts = vec![];
    for line in cart_lines {
      if let ProductVariant(variant) = &line.merchandise{
          let diskount = get_fix_discount(line.quantity, variant);
          eprintln!("var qty -> {:?}\n", line.quantity);
          // eprintln!("map -> {:?}\n", variant.qtybrks);
          eprintln!("discount -> {:?}\n", diskount);
          discounts.push(output::Discount {
              message: Some(diskount.to_string()),
              targets: vec![output::Target::ProductVariant(
                  output::ProductVariantTarget {
                      id: match line.merchandise {
                        input::InputCartLinesMerchandise::ProductVariant(variant) => variant.id,
                        _ => continue,
                    },
                      quantity: None,
                  },
              )],
              value: output::Value::FixedAmount(output::FixedAmount {
                  amount: diskount.to_string(),
                  applies_to_each_item: Some(true)
              }),
          });
        // }
      }
    }

    // The shopify_function crate serializes your function result and writes it to STDOUT
    Ok(output::FunctionResult {
        discounts: discounts,
        discount_application_strategy: output::DiscountApplicationStrategy::MAXIMUM,
    })
}



// fn get_fix_discount(quantity: i64, qbr_value: &str) -> i64 {
fn get_fix_discount(quantity: i64, variant: &InputCartLinesMerchandiseOnProductVariant) -> i64 {
  if variant.qtybrks.is_some(){
      let qbreaks_mf: Option<Value>  = 
      variant
            .qtybrks
            .as_ref()
            .map(|qtybrks| {
              qtybrks.value.parse().unwrap()
            });
      let qbmap: Value = qbreaks_mf.unwrap();
      let mut ppu = 0;
      match qbmap {
          Value::Object(obj) => {
              for (key, kval) in obj.iter() {
                  eprintln!("key -> {:?}\n", key);
                  ppu = kval.as_i64().unwrap();
                  let qkey = key.parse::<i64>().unwrap();
                  if quantity < qkey {  break; } 
              }
              return        ppu;
          }
          _ => {
              return 0;
          }
      }
    }
    return 0;
}
