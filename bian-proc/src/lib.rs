use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};

/// helper macro to create request
///
/// syntax `impl_api!(fn_name: ident method: METHOD, url: str, param_type: T, ok_resp: U)`
#[proc_macro]
pub fn impl_api(item: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(item);
    let mut input_iter = input.into_iter();
    let fn_name = input_iter.next().expect("expect function name");
    let verb = input_iter
        .next()
        .expect("expect method, GET, PUT, POST etc");
    let verb_str = verb.to_string();
    let should_sign = verb_str.starts_with("S");
    let http_method = if should_sign {
        Ident::new(&verb_str[1..].to_lowercase(), verb.span())
    } else {
        Ident::new(&verb_str.to_lowercase(), verb.span())
    };
    let url = input_iter.next().expect("expect endpoint url");
    let param_ty = if let TokenTree::Group(param_group) =
        input_iter.next().expect("expect api query params")
    {
        param_group.stream()
    } else {
        panic!("expect param type group")
    };

    let resp_ty =
        if let TokenTree::Group(resp_group) = input_iter.next().expect("expect api query params") {
            resp_group.stream()
        } else {
            panic!("expect response type group")
        };

    let should_be_none = input_iter.next();
    if should_be_none.is_some() {
        panic!("input should end here")
    }

    let expanded = if should_sign {
        quote::quote! {
            pub async fn #fn_name(&self, param: #param_ty) -> bian_core::BianResult<#resp_ty> {
                let url = dbg!(self.base_url.join(#url).unwrap());
                let qs = format!(
                    "{}&signature={}",
                    serde_qs::to_string(&param).unwrap(),
                    self.sign(&param),
                );
                let resp = self
                .http_client
                .#http_method(&format!("{}?{}", url, qs))
                .header("Content-Type", "application/json")
                .header("X-MBX-APIKEY", &self.api_key)
                .send()
                .await?;
                let resp =APIError::check_resp(resp).await?;
                let json_resp = resp.json::<#resp_ty>()
                .await
                .map_err(|e| bian_core::error::APIError::DecodeError(e.to_string()))?;
                Ok(json_resp)
            }
        }
    } else {
        quote::quote! {
            pub async fn #fn_name(&self, param: #param_ty) -> bian_core::BianResult<#resp_ty> {
                let url = dbg!(self.base_url.join(#url).unwrap());
                let qs = serde_qs::to_string(&param).unwrap();
                let resp = self
                .http_client
                .#http_method(&format!("{}?{}", url, qs))
                .header("Content-Type", "application/json")
                .header("X-MBX-APIKEY", &self.api_key)
                .send()
                .await?;
                let json_resp = resp.json::<#resp_ty>()
                .await
                .map_err(|e| bian_core::error::APIError::DecodeError(e.to_string()))?;
                Ok(json_resp)
            }
        }
    };
    TokenStream::from(expanded)
}
