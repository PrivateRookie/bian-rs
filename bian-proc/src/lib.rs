use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::{parse_macro_input, FnArg, Pat};

#[proc_macro_attribute]
pub fn api(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = syn::LitStr::new(
        &format!(" {}", &attr.to_string()),
        proc_macro2::Span::call_site(),
    );
    let mut attr_iter = proc_macro2::TokenStream::from(attr).into_iter();
    let verb = attr_iter.next().expect("expect method, GET, PUT, POST etc");
    let verb_str = verb.to_string();
    let should_sign = verb_str.starts_with("S");
    let http_method = if should_sign {
        Ident::new(&verb_str[1..].to_lowercase(), verb.span())
    } else {
        Ident::new(&verb_str.to_lowercase(), verb.span())
    };
    let url = attr_iter.next().expect("expect endpoint url");
    let mut api_fn = parse_macro_input!(item as syn::ItemFn);

    let fn_input = api_fn.sig.inputs.clone();
    let param_ident = if fn_input.len() == 1 {
        None
    } else {
        let param = fn_input.iter().skip(1).next().unwrap();
        if let FnArg::Typed(ty) = param {
            if let Pat::Ident(pat_ident) = *ty.pat.clone() {
                Some(pat_ident)
            } else {
                panic!("unknown token")
            }
        } else {
            panic!("expected param")
        }
    };
    let prepare_qs_block = if param_ident.is_some() {
        quote::quote! {
            let qs = serde_qs::to_string(&param).unwrap();
        }
    } else {
        quote::quote! {
            let qs = "";
        }
    };
    let sign_block = if should_sign {
        quote::quote! {
            let qs  = format!(
                "{}&signature={}",
                qs,
                self.sign(&param),
            );
        }
    } else {
        quote::quote! {}
    };
    let fn_block = syn::parse_quote! {

        let url = self.base_url.join(#url).unwrap();

        #prepare_qs_block

        #sign_block

        let url = if qs.is_empty() { url.to_string() } else { format!("{}?{}", url, qs) };

        let resp = self
        .http_client
        .#http_method(&url)
        .header("Content-Type", "application/json")
        .header("X-MBX-APIKEY", &self.api_key)
        .send()
        .await?;
        let resp =APIError::check_resp(resp).await?;
        let resp_text = dbg!(resp.text().await.unwrap());
        let json_resp = serde_json::from_str(&resp_text)
        .map_err(|e| crate::error::APIError::DecodeError(e.to_string()))?;
        Ok(json_resp)
    };
    api_fn.block.stmts = fn_block;
    api_fn.attrs.push(syn::parse_quote! {
        #[doc = r""]
    });
    let url_doc: syn::Attribute = syn::parse_quote! {
        #[doc = #attr_str]
    };
    api_fn.attrs.push(url_doc);
    TokenStream::from(quote::quote! { #api_fn })
}
