use alloc::rc::Weak;
use anyhow::Result;
use core::ptr::NonNull;
use log::error;
use qjs::{c, Value as JsValue};

use crate::service::{Service, ServiceRef, ServiceWeakRef};
use crate::traits::ResultExt;

pub(crate) use http_listen::try_accept_http_request;

mod debug;
mod http_listen;
mod http_request;
mod print;
mod timer;
mod url;

pub(crate) fn setup_host_functions(ctx: NonNull<c::JSContext>) -> Result<()> {
    let ns = JsValue::new_object(ctx);
    print::setup(&ns)?;
    url::setup(&ns)?;
    timer::setup(&ns)?;
    http_request::setup(&ns)?;
    http_listen::setup(&ns)?;
    debug::setup(&ns)?;
    ns.define_property_fn("close", close_res)?;
    qjs::get_global(ctx).set_property("Sidevm", &ns)?;
    Ok(())
}

#[no_mangle]
extern "C" fn __pink_getrandom(pbuf: *mut u8, nbytes: u8) {
    let buf = unsafe { core::slice::from_raw_parts_mut(pbuf, nbytes as usize) };
    crate::runtime::getrandom(buf).expect("Failed to get random bytes");
}

#[qjs::host_call(with_context)]
fn close_res(service: ServiceRef, _this: JsValue, res_id: u64) {
    service.remove_resource(res_id);
}
