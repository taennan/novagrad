use crate::{
    tui::types::{AppSystemContext, ScreenState},
    utils::{events::AppEvent, metrics::Metric},
};

pub fn run(ctx: AppSystemContext) {
    let ScreenState::ModelRun { metrics, .. } = &mut ctx.state.screen else {
        return;
    };

    match ctx.event {
        AppEvent::MetricDeleted(tag) => {
            metrics.remove(tag);
        }
        AppEvent::MetricModified(tag, metric) => {
            let existing = metrics.get_mut(tag);
            if let Some(existing) = existing {
                match (existing, metric) {
                    (Metric::Usize(a), Metric::Usize(b)) => {
                        *a = (*b).clone();
                    }
                    (Metric::F32(a), Metric::F32(b)) => {
                        *a = (*b).clone();
                    }
                    (Metric::UsizeSeries(a), Metric::UsizeSeries(b)) => {
                        a.datapoints.append(&mut b.datapoints.clone());
                    }
                    (Metric::F32Series(a), Metric::F32Series(b)) => {
                        a.datapoints.append(&mut b.datapoints.clone());
                    }
                    _ => {}
                };
            } else {
                metrics.insert(tag.clone(), (*metric).clone());
            }
        }
        _ => {}
    };
}
