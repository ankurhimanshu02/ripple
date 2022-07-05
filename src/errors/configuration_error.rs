#[derive(Debug, PartialEq)]
pub enum ConfigurationError {
	/// Empty Configuration
	EmptyConfiguration,
	/// Empty ID in Configuration
	EmptyIdInConfiguration,
	/// Empty Address in Configuration
	EmptyAddressInConfiguration,
	/// Found Duplicate ID In Configuration
	FoundDuplicateIdInConfiguration,
	/// Found Duplicate Address In Configuration
	FoundDuplicateAddressInConfiguration,
	/// Need At Least One Voter In Configuration
	NeedAtLeastOneVoterInConfiguration,
	/// Configuration changed
	ConfigurationChanged,
	/// Next Configuration Should Have Succeeded
	NextConfigurationFailed,
	/// Next Configuration Is Unexpected
	NextConfigurationUnexpected
}