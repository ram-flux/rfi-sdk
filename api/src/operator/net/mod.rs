pub(crate) mod channel;
pub(crate) mod codec;

pub(crate) fn _heartbeat() -> Result<(), crate::Error> {
    let trace_id = crate::operator::WrapWorker::worker()?.gen_trace_id()?;
    let protocol = im_codec::Protocol::heartbeat(trace_id as u64);
    Ok(())
}
