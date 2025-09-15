#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SessionRole {
	#[default]
	Undetermined,
	Applicant,
	Interviewr,
	RecruitmentManager,
}
