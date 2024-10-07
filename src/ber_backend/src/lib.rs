use ic_cdk::prelude::*;

/// Demonstrates that the system accepts best-effort calls.
/// Call this update with i > 0 to trigger a sequence of best-effort response calls
#[ic_cdk::update]
async fn tryme(i: u64) -> u64 {
    if i == 0 && ic_cdk::api::call::msg_deadline() == 0 {
        panic!("This should be a best-effort response call!")
    }
    if i > 0 {
        let res: (u64,) = Call::new(ic_cdk::id(), "tryme")
            .with_args((i - 1,))
            .change_timeout(1)
            .call()
            .await
            .expect("Didn't expect the call to fail");
        res.0
    } else {
        42
    }
}

/// Endpoint that demonstrates that timeouts can trigger with best-effort responses.
/// It calls busy() with a timeout of 1 second, which is not enough to complete the
/// execution, triggering a SYS_UNKNOWN error on the call. We return a bool to
/// the caller to indicate whether a timeout has occured (true = yes, false = no)
#[ic_cdk::update]
async fn demonstrate_timeouts() -> bool {
    let res: CallResult<(u64,)> = Call::new(ic_cdk::id(), "busy")
        .change_timeout(1)
        .with_args(((),))
        .call()
        .await;
    match res {
        Err((RejectionCode::SysUnknown, s)) => {
            ic_cdk::println!("SysUnknown: {:?}", s);
            true
        }
        Err((c, s)) => {
            ic_cdk::println!("Unexpected error returned by the call: {:?} {:?}", c, s);
            false
        }
        Ok((r,)) => {
            ic_cdk::println!("Unexpected successful result returned by the call: {:?}", r);
            false
        }
    }
}

/// Busy endpoint that just wastes a lot of instruction to trigger multi-round
/// execution, which in turn can trigger timeouts on best-effort response calls
/// to this endpoint
#[ic_cdk::update]
async fn busy() -> u64 {
    const MAX: u64 = 10_000_000_000;
    let mut x = 0;
    for _ in 0..MAX {
        x *= 2;
        x += 2;
        x /= 2;
    }
    x
}

#[ic_cdk::query]
async fn deadline_in_query() -> u64 {
    ic_cdk::api::call::msg_deadline()
}

#[ic_cdk::query(composite = true)]
async fn deadline_in_composite_query() -> u64 {
    ic_cdk::api::call::msg_deadline()
}

#[ic_cdk::query(composite = true)]
async fn test_deadlines_in_composite_query() -> (u64, u64) {
    let deadline_in_query: (u64,) = Call::new(ic_cdk::id(), "deadline_in_query")
        .with_args(((),))
        .call()
        .await
        .expect("Failed to call deadline_in_query");
    let deadline_of_query_in_composite_query: (u64,) =
        Call::new(ic_cdk::id(), "deadline_in_composite_query")
            .with_args(((),))
            .call()
            .await
            .expect("Failed to call deadline_in_composite_query");
    (deadline_in_query.0, deadline_of_query_in_composite_query.0)
}

#[ic_cdk::update]
async fn deadline_in_replicated_query() -> u64 {
    Call::new(ic_cdk::id(), "deadline_in_query")
        .with_args(((),))
        .call::<(u64,)>()
        .await
        .expect("Failed to call deadline_in_query")
        .0
}

/// Demonstrate an error when we try to call `tryme` with guaranteed response calls.
#[ic_cdk::update]
async fn demonstrate_guaranteed_responses() -> bool {
    let res: CallResult<(u64,)> = Call::new(ic_cdk::id(), "tryme")
        .with_guaranteed_response()
        .with_args((0,))
        .call()
        .await;
    match res {
        Err((RejectionCode::CanisterError, s)) => {
            ic_cdk::println!("Canister returned an error: {}", s);
            true
        }
        Err((c, s)) => {
            ic_cdk::println!("Unexpected error returned by the call: {:?} {:?}", c, s);
            false
        }
        Ok((r,)) => {
            ic_cdk::println!("Unexpected successful result returned by the call: {:?}", r);
            false
        }
    }
}
