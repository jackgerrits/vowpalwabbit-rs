use std::{
    convert::TryInto,
    ffi::CString,
    mem::MaybeUninit,
    os::raw::{c_int, c_void},
};

use vowpalwabbit_sys::{VWActionScores, VW_STATUS_SUCCESS, size_t};

use crate::{
    error::{check_panic, check_return, ErrorMessageHolder, VWError},
    example::Example,
    multi_example::MultiExample,
    prediction::Prediction,
};

pub struct Workspace {
    workspace: *mut vowpalwabbit_sys::VWWorkspace,
    error_message_holder: ErrorMessageHolder,
}

unsafe fn action_scores(pred_ptr: *mut c_void) -> Prediction {
    let mut length = MaybeUninit::<size_t>::zeroed();
    let mut error_message_holder = ErrorMessageHolder::new();
    vowpalwabbit_sys::VWActionScoresGetLength(
        pred_ptr as *const VWActionScores,
        length.as_mut_ptr(),
    );
    // todo check result
    let length = length.assume_init();
    let mut result: Vec<(u32, f32)> = Vec::new();
    for i in 0..length {
        let mut action = MaybeUninit::<u32>::zeroed();
        let mut value = MaybeUninit::<f32>::zeroed();
        let res = vowpalwabbit_sys::VWActionScoresGetValue(
            pred_ptr as *const VWActionScores,
            action.as_mut_ptr(),
            value.as_mut_ptr(),
            i,
            error_message_holder.get_mut_ptr(),
        );
        check_panic!(res, error_message_holder);
        result.push((action.assume_init(), value.assume_init()));
    }
    vowpalwabbit_sys::VWActionScoresDelete(pred_ptr as *mut VWActionScores);
    Prediction::ActionScores { values: result }
}

impl Workspace {
    // TODO use a trait bound of something to make this a more flexible input, &str, &String, String etc
    pub fn new(args: &[String]) -> Result<Workspace, VWError> {
        let mut workspace: *mut vowpalwabbit_sys::VWWorkspace = std::ptr::null_mut();

        let args = args
            .iter()
            .map(|arg| CString::new(arg.clone()).unwrap())
            .collect::<Vec<CString>>();

        let c_args = args
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const ::std::os::raw::c_char>>();

        let mut error_message_holder = ErrorMessageHolder::new();
        unsafe {
            let res = vowpalwabbit_sys::VWWorkspaceInitialize(
                c_args.as_ptr(),
                c_args.len() as c_int,
                &mut workspace,
                error_message_holder.get_mut_ptr(),
            );
            check_return!(res, error_message_holder);
            Ok(Workspace {
                workspace,
                error_message_holder,
            })
        }
    }

    fn get_ptr(&self) -> *const vowpalwabbit_sys::VWWorkspace {
        self.workspace
    }

    fn get_mut_ptr(&mut self) -> *mut vowpalwabbit_sys::VWWorkspace {
        self.workspace
    }

    pub fn learn(&mut self, example: &mut Example) -> Result<(), VWError> {
        self.error_message_holder.clear();
        unsafe {
            let res = vowpalwabbit_sys::VWWorkspaceLearn(
                self.get_mut_ptr(),
                example.get_mut_ptr(),
                self.error_message_holder.get_mut_ptr(),
            );
            check_return!(res, self.error_message_holder);
        }
        Ok(())
    }

    pub fn learn_multi_example(&mut self, example: &mut MultiExample) -> Result<(), VWError> {
        self.error_message_holder.clear();
        unsafe {
            // TODO check result
            let res = vowpalwabbit_sys::VWWorkspaceLearnMultiEx(
                self.get_mut_ptr(),
                example.get_mut_ptr(),
                self.error_message_holder.get_mut_ptr(),
            );
            check_return!(res, self.error_message_holder);
            Ok(())
        }
    }

    pub fn predict(&mut self, example: &mut Example) -> Result<Prediction, VWError> {
        self.error_message_holder.clear();
        unsafe {
            // TODO check result
            let mut prediction = MaybeUninit::<*mut c_void>::zeroed();
            let mut prediction_type = MaybeUninit::<u32>::zeroed();
            let res = vowpalwabbit_sys::VWWorkspacePredict(
                self.get_mut_ptr(),
                example.get_mut_ptr(),
                prediction.as_mut_ptr(),
                prediction_type.as_mut_ptr(),
                self.error_message_holder.get_mut_ptr(),
            );
            check_return!(res, self.error_message_holder);
            let prediction_type = prediction_type.assume_init();
            let prediction = prediction.assume_init();
            match prediction_type {
                vowpalwabbit_sys::override_prediction_type_t_action_scores => {
                    Ok(action_scores(prediction))
                }
                _ => Err(VWError::Failure("Unknown".to_string())),
            }
        }
    }

    pub fn predict_multi_example(
        &mut self,
        example: &mut MultiExample,
    ) -> Result<Prediction, VWError> {
        self.error_message_holder.clear();
        unsafe {
            // TODO check result
            let mut prediction = MaybeUninit::<*mut c_void>::zeroed();
            let mut prediction_type = MaybeUninit::<u32>::zeroed();
            let res = vowpalwabbit_sys::VWWorkspacePredictMultiEx(
                self.get_mut_ptr(),
                example.get_mut_ptr(),
                prediction.as_mut_ptr(),
                prediction_type.as_mut_ptr(),
                self.error_message_holder.get_mut_ptr(),
            );
            check_return!(res, self.error_message_holder);
            let prediction_type = prediction_type.assume_init();
            let prediction = prediction.assume_init();
            match prediction_type {
                vowpalwabbit_sys::override_prediction_type_t_action_scores => {
                    Ok(action_scores(prediction))
                }
                _ => Err(VWError::Failure("Unknown".to_string())),
            }
        }
    }

    pub fn parse_decision_service_json(&self, content: &str) -> Result<MultiExample, VWError> {
        unsafe {
            let mut error_message_holder = ErrorMessageHolder::new();
            let multi_ex_handle = vowpalwabbit_sys::VWMultiExCreate();
            let res = vowpalwabbit_sys::VWWorkspaceParseDSJson(
                self.get_ptr(),
                content.as_ptr() as *const i8,
                content.len().try_into().unwrap(),
                multi_ex_handle,
                error_message_holder.get_mut_ptr(),
            );
            check_return!(res, error_message_holder);
            Ok(MultiExample {
                multi_example: multi_ex_handle,
            })
        }
    }

    pub fn setup_ex(&self, example: &mut Example) -> Result<(), VWError> {
        unsafe {
            let mut error_message = ErrorMessageHolder::new();
            let res = vowpalwabbit_sys::VWWorkspaceSetupExample(
                self.get_ptr(),
                example.get_mut_ptr(),
                error_message.get_mut_ptr(),
            );
            check_return!(res, error_message);
            Ok(())
        }
    }

    pub fn setup_multi_ex(&self, examples: &mut MultiExample) -> Result<(), VWError> {
        unsafe {
            let mut error_message = ErrorMessageHolder::new();
            let res = vowpalwabbit_sys::VWWorkspaceSetupMultiEx(
                self.get_ptr(),
                examples.get_mut_ptr(),
                error_message.get_mut_ptr(),
            );
            check_return!(res, error_message);
            Ok(())
        }
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::VWWorkspaceDelete(self.get_mut_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::workspace::Workspace;

    #[test]
    fn create_workspace() {
        let args: Vec<String> = vec!["--quiet".to_owned()];
        let maybe_workspace = Workspace::new(&args);
        assert!(maybe_workspace.is_ok());
    }

    #[test]
    fn create_workspace_with_invalid_option() {
        let args: Vec<String> = vec!["--not_real".to_owned()];
        let maybe_workspace = Workspace::new(&args);
        assert!(maybe_workspace.is_err());
    }

    #[test]
    fn parse_dsjson() {
        let args: Vec<String> = vec!["--quiet".to_owned(), "--cb_explore_adf".to_owned()];
        let workspace = Workspace::new(&args).unwrap();
        let mut examples = workspace.parse_decision_service_json(r#"{"_label_cost":-0.0,"_label_probability":0.05000000074505806,"_label_Action":4,"_labelIndex":3,"o":[{"v":0.0,"EventId":"13118d9b4c114f8485d9dec417e3aefe","ActionTaken":false}],"Timestamp":"2021-02-04T16:31:29.2460000Z","Version":"1","EventId":"13118d9b4c114f8485d9dec417e3aefe","a":[4,2,1,3],"c":{"FromUrl":[{"timeofday":"Afternoon","weather":"Sunny","name":"Cathy"}],"_multi":[{"_tag":"Cappucino","i":{"constant":1,"id":"Cappucino"},"j":[{"type":"hot","origin":"kenya","organic":"yes","roast":"dark"}]},{"_tag":"Cold brew","i":{"constant":1,"id":"Cold brew"},"j":[{"type":"cold","origin":"brazil","organic":"yes","roast":"light"}]},{"_tag":"Iced mocha","i":{"constant":1,"id":"Iced mocha"},"j":[{"type":"cold","origin":"ethiopia","organic":"no","roast":"light"}]},{"_tag":"Latte","i":{"constant":1,"id":"Latte"},"j":[{"type":"hot","origin":"brazil","organic":"no","roast":"dark"}]}]},"p":[0.05,0.05,0.05,0.85],"VWState":{"m":"ff0744c1aa494e1ab39ba0c78d048146/550c12cbd3aa47f09fbed3387fb9c6ec"},"_original_label_cost":-0.0}"#).unwrap();
        workspace.setup_multi_ex(&mut examples).unwrap();
    }

    #[test]
    fn parse_invalid_dsjson() {
        let args: Vec<String> = vec!["--cb_explore_adf".to_owned()];
        let workspace = Workspace::new(&args).unwrap();
        let maybe_examples = workspace.parse_decision_service_json(r#"{"unclosed}"#);
        assert!(maybe_examples.is_err());
    }

    #[test]
    fn parse_dsjson_and_learn() {
        let args: Vec<String> = vec!["--quiet".to_owned(), "--cb_adf".to_owned()];
        let mut workspace = Workspace::new(&args).unwrap();
        let mut examples = workspace.parse_decision_service_json(r#"{"_label_cost":-1.0,"_label_probability":0.05000000074505806,"_label_Action":4,"_labelIndex":3,"o":[{"v":0.0,"EventId":"13118d9b4c114f8485d9dec417e3aefe","ActionTaken":false}],"Timestamp":"2021-02-04T16:31:29.2460000Z","Version":"1","EventId":"13118d9b4c114f8485d9dec417e3aefe","a":[4,2,1,3],"c":{"FromUrl":[{"timeofday":"Afternoon","weather":"Sunny","name":"Cathy"}],"_multi":[{"_tag":"Cappucino","i":{"constant":1,"id":"Cappucino"},"j":[{"type":"hot","origin":"kenya","organic":"yes","roast":"dark"}]},{"_tag":"Cold brew","i":{"constant":1,"id":"Cold brew"},"j":[{"type":"cold","origin":"brazil","organic":"yes","roast":"light"}]},{"_tag":"Iced mocha","i":{"constant":1,"id":"Iced mocha"},"j":[{"type":"cold","origin":"ethiopia","organic":"no","roast":"light"}]},{"_tag":"Latte","i":{"constant":1,"id":"Latte"},"j":[{"type":"hot","origin":"brazil","organic":"no","roast":"dark"}]}]},"p":[0.05,0.05,0.05,0.85],"VWState":{"m":"ff0744c1aa494e1ab39ba0c78d048146/550c12cbd3aa47f09fbed3387fb9c6ec"},"_original_label_cost":-0.0}"#).unwrap();
        workspace.setup_multi_ex(&mut examples).unwrap();
        workspace.learn_multi_example(&mut examples).unwrap();
        workspace.learn_multi_example(&mut examples).unwrap();
        workspace.learn_multi_example(&mut examples).unwrap();
        let pred = workspace.predict_multi_example(&mut examples).unwrap();
        println!("{:#?}", pred);
    }
}
