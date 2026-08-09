use crate::{
    models::exponential_predictor_v2::ExpPredictor,
    tui::types::{AppSystem, ModelRunState, ModelSelectWizard, RunMode, ScreenState, WizardStep},
};
use crossterm::event::KeyCode;

/// Handles user input for the model select wizard:
/// - Up/Down arrows navigate the list
/// - Enter confirms the current selection and advances the wizard
/// - Backspace goes back to the previous step
pub fn new() -> AppSystem {
    Box::new(|state| {
        let ScreenState::ModelSelect { wizard } = &mut state.screen else {
            return;
        };

        if state.keys_pressed.contains(&KeyCode::Up) {
            move_cursor(wizard, -1);
        } else if state.keys_pressed.contains(&KeyCode::Down) {
            move_cursor(wizard, 1);
        } else if state.keys_pressed.contains(&KeyCode::Enter) {
            match go_forward(wizard) {
                Some(screen) => state.screen = screen,
                _ => {}
            }
        } else if state.keys_pressed.contains(&KeyCode::Backspace) {
            match go_back(wizard) {
                Some(screen) => state.screen = screen,
                _ => {}
            }
        }

        state.keys_pressed.clear();
    })
}

fn move_cursor(wizard: &mut ModelSelectWizard, delta: i32) {
    let list_len = match wizard.step {
        WizardStep::SelectModel => wizard.models.len(),
        WizardStep::SelectRunMode => wizard.run_modes.len(),
        WizardStep::SelectDataset => wizard.datasets.len(),
        WizardStep::Confirm => 0,
    };

    if list_len == 0 {
        return;
    }

    let current = wizard.cursor as i32;
    let new_cursor = (current + delta).clamp(0, (list_len - 1) as i32) as usize;
    wizard.cursor = new_cursor;
}

fn go_forward(wizard: &mut ModelSelectWizard) -> Option<ScreenState> {
    match wizard.step {
        WizardStep::SelectModel => {
            wizard.selected_model = Some(wizard.cursor);
            wizard.step = WizardStep::SelectRunMode;
            wizard.cursor = 0;
            None
        }
        WizardStep::SelectRunMode => {
            wizard.selected_run_mode = Some(wizard.cursor);
            wizard.step = WizardStep::SelectDataset;
            wizard.cursor = 0;
            None
        }
        WizardStep::SelectDataset => {
            wizard.selected_dataset = Some(wizard.cursor);
            wizard.step = WizardStep::Confirm;
            wizard.cursor = 0;
            None
        }
        WizardStep::Confirm => {
            let model = Box::new(ExpPredictor::new());
            let run = ModelRunState::mocked();
            let mode = match wizard.selected_run_mode {
                Some(selected) => wizard.run_modes[selected],
                None => RunMode::default(),
            };

            let screen = ScreenState::ModelRun { mode, run };
            Some(screen)
        }
    }
}

fn go_back(wizard: &mut ModelSelectWizard) -> Option<ScreenState> {
    match wizard.step {
        WizardStep::SelectModel => {
            return Some(ScreenState::Home);
        }
        WizardStep::SelectRunMode => {
            wizard.step = WizardStep::SelectModel;
            wizard.selected_run_mode = None;
            wizard.cursor = wizard.selected_model.unwrap_or(0);
        }
        WizardStep::SelectDataset => {
            wizard.step = WizardStep::SelectRunMode;
            wizard.selected_dataset = None;
            wizard.cursor = wizard.selected_run_mode.unwrap_or(0);
        }
        WizardStep::Confirm => {
            wizard.step = WizardStep::SelectDataset;
            wizard.cursor = wizard.selected_dataset.unwrap_or(0);
        }
    }

    None
}
