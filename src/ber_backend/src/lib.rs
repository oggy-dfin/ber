use ic_cdk::prelude::*;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}


#[ic_cdk::update]
async fn tryme(i: u64) -> u64 {
    if i == 0 && ic_cdk::api::call::msg_deadline() == 0 {
	panic!("This should be a best-effort call!")
    }
    if i > 0 {
    	let res: (u64,) = Call::new(ic_cdk::id(), "tryme")
    	    .with_args((i - 1, ))
    	    .change_timeout(1)
    	    .call()
    	    .await
    	    .expect("Didn't expect the call to fail");
    	res.0
    } else {
      42
    }
}
