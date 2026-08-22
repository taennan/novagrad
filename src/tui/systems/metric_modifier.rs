use crate::utils::{
    Primitive,
    events::AppEvent,
    metrics::{Datapoint, Metric},
    state::ScreenState,
    system::AppSystemContext,
};

pub fn run(ctx: AppSystemContext) {
    let ScreenState::ModelRun { metrics, .. } = &mut ctx.state.screen else {
        return;
    };

    match ctx.event {
        AppEvent::MetricAdded(tag, metric) => {
            metrics.insert(tag, metric.clone());
        }
        AppEvent::MetricDeleted(tag) => {
            metrics.remove(tag);
        }
        AppEvent::MetricModified(tag, primtive) => {
            let existing = metrics.get_mut(tag);
            if let Some(existing) = existing {
                match (existing, primtive) {
                    (Metric::Usize(a), Primitive::Usize(b)) => {
                        (*a).value = *b;
                    }
                    (Metric::F32(a), Primitive::F32(b)) => {
                        (*a).value = *b;
                    }
                    (Metric::UsizeSeries(a), Primitive::Usize(b)) => {
                        a.datapoints.push(Datapoint::now(*b));
                    }
                    (Metric::F32Series(a), Primitive::F32(b)) => {
                        a.datapoints.push(Datapoint::now(*b));
                    }
                    _ => {}
                };
            }
        }
        _ => {}
    };
}
