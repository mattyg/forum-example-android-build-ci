use hdi::prelude::*;

pub trait ValidateEntry {
    fn validate_create(action: EntryCreationAction, entry: Self) -> ExternResult<ValidateCallbackResult> {
        Ok(ValidateCallbackResult::Valid)
    }

    fn validate_update(
        action: Update,
        entry: Self,
        original_action: EntryCreationAction
    ) -> ExternResult<ValidateCallbackResult> {
        Ok(ValidateCallbackResult::Valid)
    }

    fn validate_delete(
        action: Delete,
        original_action: EntryCreationAction,
        original_post: Self,
    ) -> ExternResult<ValidateCallbackResult> {
        Ok(ValidateCallbackResult::Valid)
    }

    fn validate_record_exists_and_contains_proper_entry_type(action_hash: ActionHash) -> ExternResult<ValidateCallbackResult> {
        let original_app_entry = must_get_valid_record(
            action_hash
        )?;
        let original_entry_data = match Self::try_from(
            original_app_entry,
        ) {
            Ok(entry) => entry,
            Err(e) => {
                return Ok(
                    ValidateCallbackResult::Invalid(
                        format!("Expected to get {Self:?} from Record: {e:?}"),
                    ),
                );
            }
        };
    }
}