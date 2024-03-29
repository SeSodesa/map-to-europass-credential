#![cfg_attr(debug_assertions, allow(dead_code))]
/*!
This submodule defines the Europass credential type EuropassCredential
and the types required in the construction of its fields.

See https://github.com/european-commission-europass/Europass-Learning-Model/blob/master/Europass_Learning_Model.md
*/

use chrono;
use crate::national_qualification_frameworks as nqf;
use crate::european_qualifications_framework as eqf;
use crate::controlled_vocabularies;

/// A set of one or more claims made by an issuer.
/// A credential is a set of one or more claims made by the same entity.
/// A verifiable credential is a tamper-evident credential that has
/// authorship that can be cryptographically verified.
/// Verifiable credentials can be used to build verifiable presentations,
/// which can also be cryptographically verified.
struct VerifiableCredential {
    /// A unique portable identifier of the credential.
    /// Has to be a valid URI.
    id: URI,
    /// The issuance date of the credential.
    issuance_date: chrono::naive::NaiveDateTime,
    /// The date and time the credential was digitally signed.
    issued: chrono::naive::NaiveDateTime,
    /// The earliest date when the information associated with
    /// the credentialSubject property became valid.
    valid_from: chrono::naive::NaiveDateTime,
    /// The expiration date of this credential.
    expiration_date: chrono::naive::NaiveDateTime,
    /// The Europass Credential carried within this verifiable credential.
    europass_credential: EuropassCredential,
}

/// A set of claims made by an issuer in Europe, using the Europass Standards.
/// A Europass credential is a set of one or more claims which may be used to
/// demonstrate that the owner has certain skills or has achieved certain
/// learning outcomes through formal, non-formal or informal learning.
pub struct EuropassCredential {
    /// The identifier of this Europass Credential.
    identifier: Identifier,
    /// The type of this credential.
    /// See https://op.europa.eu/en/web/eu-vocabularies/concept-scheme/-/resource?uri=http://data.europa.eu/snb/credential/25831c2
    /// for details.
    credential_type: controlled_vocabularies::CredentialType,
    /// The full official title of the issued credential
    /// (maximum 50 characters).
    title: Text,
    /// A summary of the claim or group of claims being
    /// made about a person (maximum 140 words).
    description: Note,
    /// The organisaton that issued the credential and
    /// sealed it with their digital e-seal.
    issuer: Organisation,
    /// The person about which claims are made and who owns the credential.
    credential_subject: Person,
    /// The display details of the credential.
    display: String,
    /// Any digital document (PDF, JPEG or PNG format) that an issuer
    /// has attached to the Europass document.
    attachment: EuropassAttachment,
    /// The cryptographic proofs that can be used to detect tampering and
    /// verify the authorship of a credential or presentation.
    proof: Proof,
    /// A credential embedded within the credential.
    /// Smaller sub-credentials (micro-credentials),
    /// that make up this larger credential when combined.
    contains: Option<Box<EuropassCredential>>,
}

/// The cryptographic proof that can be used to detect tampering and
/// verify the authorship of a credential or presentation.
struct Proof {
    /// The code indicating how to display the summary view of the credential.
    display_code: String,
    /// The background image of the credential.
    background: ImageObject
}

/// An abstract entity that is able to carry out actions.
struct Agent {
    /// A portable identifier of the agent.
    id: URI,
    /// A formally issued identifier of the agent.
    identifier: Identifier,
    /// The Type of an Agent as described in a controlled vocabulary.
    agent_type: Code,
    /// The primary name of the agent.
    preferred_name: Text,
    /// An agent may have any number of alternative or informal names.
    alternative_name: Text,
    /// An additional free text note about the agent.
    note: Note,
    /// The contact information of an agent.
    contact_point: ContactInformation,
}

/// A concrete human instance of an agent.
struct Person {
    /// The unique and portable identifier of the person.
    id: URI,
    /// The "primary" national identifier of the person.
    national_id: LegalIdentifier,
    /// An (optional) alternative formally-issued identifier for the person,
    /// e.g. social security number, student ID card number, to club membership, etc.
    identifier: Identifier,
    /// The complete name of the person as one string.
    full_name: Text,
    /// The given name(s) of the person.
    given_names: Text,
    /// The family name of the person.
    family_name: Text,
    /// The name of the person at birth.
    /// Birth names tend to be persistent and for this reason
    /// they are recorded by some public sector information systems.
    /// There is no granularity for birth name - the full name should
    /// be recorded in a single field.
    birth_name: Text,
    /// Patronymic names are important in some countries.
    /// Iceland does not have a concept of 'family name' in the way
    /// that many other European countries do, for example,
    /// Erik Magnusson and Erika Magnusdottir are siblings,
    /// both offspring of Mangnus, irrespective of his patronymic name.
    /// In Bulgaria and Russia, patronymic names are in every day usage,
    /// for example, the Sergeyevich in 'Mikhail Sergeyevich Gorbachev.'
    patronymic_name: Text,
    /// The birth date of the person.
    date_of_birth: chrono::naive::NaiveDate,
    /// The place of birth of the person.
    place_of_birth: Location,
    /// The gender of the person.
    gender: Code,
    /// The country (or countries) that conferred citizenship
    /// rights on the person.
    citizenship_country: Code,
    /// A location related to a Person.
    /// For example a person's home or residence location,
    /// a person's work place location,
    /// site location of an organisation, etc.
    has_location: Location,
    /// A learning activity that a person participated in or attended.
    performed: LearningActivity,
    /// An achievement of the person.
    achieved: LearningAchievement,
    /// The entitlement of the person.
    entitled_to: Entitlement,
}

/// A concrete instance of an Agent.
/// A legal person / registered organisation.
struct Organisation {
    /// The unique and portable identifier of the organisation.
    id: URI,
    /// Another formally-issued identifier for the organisation.
    identifier: Identifier,
    /// The official identification number of the organisation,
    /// as awarded by the relevant national authority.
    ///
    /// See chapter 5.1.4 in Draft ETSI EN 319 412-1 V1.4.2:
    /// https://www.etsi.org/deliver/etsi_en/319400_319499/31941201/01.04.02_20/en_31941201v010402a.pdf
    eidas_legal_identifier: Identifier,
    /// The legal identifier of an organization.
    /// The identifier given to a registered organization by the authority
    /// with which it is registered. The legal status of
    /// a registered organization is conferred on it by an authority
    /// within a given jurisdiction. The Legal Identifier is therefore
    /// a fundamental relationship between an organization and the authority
    /// with which it is registered.
    registration: Identifier,
    /// The Value-Added Tax ID.
    vat_identifier: Identifier,
    /// Fiscal ID of the organisation.
    tax_identifier: Identifier,
    /// The primary name of the organisation.
    preferred_name: Text,
    /// An (optional) alternative name of the organisation as typically
    /// used in documents, including credentials.
    alternative_name: Text,
    /// A homepage of the organisation.
    home_page: WebDocument,
    /// The legally registered site of the organisation.
    has_location: Location,
    /// Accreditation Records associated with the organisation.
    /// More information about the accreditation database is available here.
    has_accreditation: Accreditation,
    /// A smaller organisation of which forms part of this organisation,
    /// e.g. a Department within a larger Organisation.
    has_unit: Box<Organisation>,
    /// Indicates a larger Organisation of which this Unit is a part of,
    /// e.g. the Organisation within which a Department operates.
    unit_of: Box<Organisation>,
    /// The logo of the organisation.
    logo: ImageObject,
}

/// Details to Contact an Agent. A contact point for an agent.
struct ContactInformation {
    /// A note about the contactpoint
    /// (e.g. availibility or usage note)
    note: Note,
    /// A free text describing the contact details.
    description: Note,
    /// A postal address used for contacting the agent.
    postal_address: Address,
    /// A phone number used for contacting the agent.
    phone: Phone,
    /// An e-mail address used for contacting the agent.
    email: MailBox,
    /// The wallet address of the agent.
    wallet_address: MailBox,
    /// A contact form used for contacting the agent.
    contact_form: InteractiveWebResource,
}

/// The quality assurance or licensing of an organisation or a qualification.
/// An accreditation instance can be used to specify information about:
///
/// *   the quality assurance and/or licensing of an organisation.
/// *   the quality assurance and/or licensing of an organisation
///     with respect to a specific qualification.
struct Accreditation {
    /// The portable and unique identifier of the accreditation record.
    id: URI,
    /// An alternative Identifier of the Accreditation,
    /// as assigned to it by the accrediting agent.
    identifier: Identifier,
    /// The type of accreditation.
    accreditation_type: controlled_vocabularies::AccreditationType,
    /// The title of the accreditation.
    title: Text,
    /// A description of this accreditation.
    description: Note,
    /// The Quality Decision issued by the Quality Assuring Authority.
    decision: TextualScore,
    /// A publicly accessible report of the quality assurance decision.
    report: WebDocument,
    /// The organisation whose activities are being accredited.
    organisation: Box<Organisation>,
    /// The qualification that was accredited.
    limit_qualification: Qualification,
    /// The field of education for which the accreditation is valid.
    limit_field: Code,
    /// The european qualification level for which the accreditation is valid.
    limit_eqf_level: eqf::EQFLevel,
    /// The jurisdiction for which the accreditation is valid.
    limit_jurisdiction: Code,
    /// The Quality Assuring Authority. (i.e assurer)
    accrediting_agent: Box<Organisation>,
    /// The date when the accreditation was formally approved/issued.
    issue_date: chrono::naive::NaiveDateTime,
    /// The date when the accreditation has to be re-viewed.
    review_date: chrono::naive::NaiveDateTime,
    /// The date when the accreditation expires or was expired.
    expiry_date: chrono::naive::NaiveDateTime,
    /// An additional free text note about the accreditation.
    additional_note: Note,
    /// The homepage of the accreditation.
    home_page: WebDocument,
    /// The landingpage of the accreditation.
    landing_page: WebDocument,
    /// A public web document containing additional
    /// documentation describing the Accreditation Procedures and Standards
    supplementary_document: WebDocument,
}

/// A verifiable presentation of a set of credentials.
/// A composition of a set of credentials that can be presented to
/// and verified by a verifier.
struct VerifiablePresentation {
    /// A portable identifier of the presentation.
    id: URI
}

/// A verifiable presentation of a set of Europass credentials.
struct EuropassPresentation {
    /// A verifiable EuropassCredential.
    verfiable_credential: EuropassCredential,
    ///
    verification_check: VerificationCheck,
    /// The cryptographic proof that can be used
    /// to detect tampering and verify the authorship of a presentation.
    proof: Proof,
}

/// A verification check.
struct VerificationCheck {
    /// The portable and unique identifier of the verification check
    id: URI,
    /// The type of verification check.
    /// One of Europass standard list of verification types.
    check_type: controlled_vocabularies::VerificationType,
    /// The credential subject of this verififcation check.
    subject: EuropassCredential,
    /// The result of the check.
    status: controlled_vocabularies::VerificationStatus,
    /// A free text description of the check and the result.
    description: Note,
}

/// The types of attachments that might come with a Europass credential.
enum EuropassAttachment {
    /// Portable Document Format
    PDF,
    /// Portable Network Graphics
    PNG,
    /// JPEG
    JPEG,
}

/// A description of what a person may learn using the opportunity,
/// expressed as learning outcomes. A specification of learning.
struct LearningSpecification {
    /// A portable and unique identifier of the learning specification.
    id: URI,
    /// An alternative identifier of the learning specification,
    /// as assigned to it by the organisation who designed the specification.
    identifier: Identifier,
    /// The type of learning opportunity.
    learning_opportunity_type: Code,
    /// The title of the learning specification.
    title: Text,
    /// An alternative name of the learning specification.
    alternative_labe: Text,
    /// Short and abstract description about the learning specification.
    definition: Note,
    /// The full learning outcome description of the learning specification.
    learning_outcome_description: Note,
    /// An additional free text note about the learning specification.
    addtional_note: Note,
    /// The homepage (a public web document) of the learning specification.
    home_page: WebDocument,
    /// A public web document containing additional documentation
    /// about the learning specification.
    supplemenary_document: WebDocument,
    /// Thematic Area according to the ISCED-F 2013 Classification
    iscedfc_code: Code,
    /// An associated field of education from another
    /// semantic framework than the ISCED classification.
    education_subject: EducationSubjectAssociation,
    /// The estimated number of hours the learner is expected to spend
    /// engaged in learning to earn the award. This would include
    /// the notional number of hours in class, in group work, in practicals,
    /// as well as hours engaged in self-motivated study.
    volume_of_learning: Duration,
    /// The credit points assigned to the learning specification,
    /// following the ECTS credit system.
    ects_credit_points: NumericScore,
    /// The credit points assigned to the learning specification,
    /// following an alternative educational credit system.
    credit_points: NumericScore,
    /// An associated level of education within a semantic framework
    /// describing education levels.
    education_level: EducationLevelAssociation,
    /// The instruction and/or assessment language(s) used.
    language: crate::controlled_vocabularies::Language,
    /// The mode of learning and or assessment.
    mode: crate::controlled_vocabularies::ModeOfLearningType,
    /// The type of learning setting (formal, non-formal).
    learning_setting: crate::controlled_vocabularies::LearningSettingType,
    /// The maximum duration (in months) that a person may use
    /// to complete the learning opportunity.
    maximum_duration: Duration,
    /// A specific target group or category for which this specification is designed.
    target_group: crate::controlled_vocabularies::LearningTargetGroup,
    /// Specific entry requirements or prerequisites of individuals
    /// for which this specification is designed to start this learning opportunity.
    entry_requirements_note: Note,
    /// An individual (expected) learning outcome of the learning specification.
    learning_outcome: LearningOutcome,
    /// Activities which a person can perform to acquire
    /// the expected learning outcomes.
    learning_activity_specification: LearningActivitySpecification,
    /// Assessments a person can undergo to prove
    /// the acquisition of the learning outcomes
    assessment_sppecification: AssessmentSpecification,
    /// Rights, such as which the person may acquire as
    /// a result of acquiring the learning outcomes.
    entitlement_specification: EntitlementSpecification,
    /// Refers to an activity related to the awarding of
    /// the learning specification, such as the country or region
    /// where the qualifi-cation is awarded, the awarding body and
    /// optionally the awarding period now or in the past.
    awarding_opportunity: AwardingOpportunity,
    /// A learning specification can be composed of other "narrower"
    /// learning specifications which when combined make up this
    /// learning specification.
    has_part: Box<LearningSpecification>,
    /// A learning specification (e.g. a standard) of which
    /// this specification is a specialisation.
    ///
    /// TODO: To be imlemented at a later stage.
    specialisation_of: Box<LearningSpecification>,
}

/// A specification of an assessment and validation process which is
/// obtained when a competent authority determines that an individual
/// has achieved learning outcomes to given standards.
struct Qualification {
    /// Indicates whether a qualification is a
    /// full qualification or part of another qualification.
    is_partial_qualification: IndicatorType,
    /// The qualification level as specified by
    /// the European Qualification Framework.
    eqf_level: eqf::EQFLevel,
    /// The qualification level as specified by
    /// a National Qualification Framework.
    nqf_level: nqf::NQF,
    /// The accreditation of a qualification.
    has_accreditation: Box<Accreditation>,
}

/// A statement regarding what a learner knows, understands and is able
/// to do on completion of a learning process, which are defined in terms
/// of knowledge, skills and responsibility and autonomy.
struct LearningOutcome {
    /// A portable and unique identifier of the learning outcome.
    id: URI,
    /// An alternative identifier of the learning outcome.
    identifier: Identifier,
    /// A legible, descriptive name for the learning outcome.
    name: Text,
    /// A free text describing the learning outcome.
    /// A detailed learning outcome may include a description of what
    /// the student can do as a result of learning, with an indication
    /// of the level of achievement, and the conditions or context under
    /// which this can be performed (if applicable).
    description: Note,
    /// The learning outcome type.
    learning_outcome_type: Code,
    /// The reusability level.
    reusability_level: Code,
    /// A link to a related skill or the level of
    /// a related skill on a skill framework (except ESCO).
    related_skill: Code,
    /// A link to an ESCO Skill.
    related_esco_skill: Code,
}

/// The specification of a process which leads to the acquisition of knowledge,
/// skills or responsibility and autonomy.
struct LearningActivitySpecification {
    /// A portable and unique identifier of the learning activity specification.
    id: URI,
    /// An alternative identifier of the Learning Activity,
    /// as assigned to it by the organisation who designed the specification.
    identifier: Identifier,
    /// The title of the learning activity specification.
    title: Text,
    /// An alternative name of the activity specification.
    alternative_label: Text,
    /// A free text description of the learning activity specification.
    description: Note,
    /// An additional free text note about
    /// the learning activity specification.
    additional_note: Note,
    /// Webpage describing the activity specification.
    home_page: WebDocument,
    /// A public web document containing additional
    /// documentation about the learning activity specification.
    supplementary_document: WebDocument,
    /// The type of activity.
    learning_activity_type: Code,
    /// The expected workload indicated in the estimated
    /// number of hours the learner is expected to spend
    /// engaged in the activity. This would include
    /// the notional number of hours in class, in group work,
    /// in practicals, as well as hours engaged in self-motivated study.
    workload: Duration,
    /// The instruction language(s) used.
    language: Code,
    /// The mode of learning and or assessment.
    mode: Code,
    /// The expected learning outcomes this learning activity
    /// specification can lead or contribute to.
    teaches: Box<LearningSpecification>,
    /// A learning activity specification can be composed of
    /// smaller learning specifications, which when combined
    /// make up this learning specification.
    has_part: Box<LearningActivitySpecification>,
    /// An activity specification (e.g. a standard)
    /// of which this specification is a specialisation.
    specialisation_of: Box<LearningActivitySpecification>,
}

/// Any process which leads to the acquisition of knowledge,
/// skills or responsibility and autonomy.
struct LearningActivity {
    /// A portable and unique identifier of the learning activity.
    id: URI,
    /// An alternative identifier of the learning activity assigned
    /// to the assessment by the organisation directing the activity.
    identifier: Identifier,
    /// The title of the learning activity.
    title: Text,
    /// A free text description of the learning activity.
    description: Note,
    /// An additional free text note about the activity.
    additional_note: Note,
    /// The actual workload in number of hours the learner
    /// has spent engaged in the activity. This would include
    /// the number of hours in class, in group work, in practicals,
    /// as well as hours engaged in self-motivated study.
    workload: Duration,
    /// The date the learner started the activity
    started_at_time: chrono::naive::NaiveDateTime,
    /// The date the learner ended the activity
    ended_at_time: chrono::naive::NaiveDateTime,
    /// The organisation, or part of an organisation such as department,
    /// faculty, which directed the learning activity.
    directed_by: Agent,
    /// The location where the activity took place
    location: Location,
    /// The specification of this learning activity.
    specified_by: LearningActivitySpecification,
    /// The used or taken opportunity to do this learning activity.
    used_learning_opportunity: Box<LearningOpportunity>,
    /// Performing this activity contributed to the acquisition
    /// of these related learning achievements.
    influenced: Box<Achievement>,
    /// Smaller units of activity, which when combined make up this activity.
    has_part: Box<LearningActivity>,
}

struct Achievement;
struct LearningOpportunity;

/// An Assessment Specification is a specification of a process establishing
/// the extent to which a learner has attained particular knowledge,
/// skills and competences against criteria such as learning outcomes or
/// standards of competence.
struct AssessmentSpecification {
    /// A portable and Unique Identifier of the Assessment Specification
    id: URI,
    /// An alternative identifier of the assessment specification,
    /// as assigned to it by the organisation who designed the specification.
    identifier: Identifier,
    /// The title of the assessment specification.
    title: Text,
    /// An alternative name of the assessment specification.
    alternative_label: Text,
    /// A free text description of the assessment specification.
    description: Note,
    /// An additional free text note about the assessment specification.
    additional_note: Note,
    /// The homepage (a public web document) describing
    /// the details of the assessment specification
    home_page: WebDocument,
    /// A public web document containing additional documentation
    /// about the assessment specification.
    supplementary_document: WebDocument,
    /// The type of assessment.
    assessment_type: controlled_vocabularies::AssessmentType,
    /// The language(s) of assessment used.
    language: Code,
    /// The mode of learning and or assessment.
    mode: Code,
    /// A description of the specification of which
    /// learning outcomes are or have been proven.
    grading_scheme: ScoringScheme,
    /// The learning achievement (and related learning outcomes)
    /// this assessment is designed to test.
    proves: Box<LearningSpecification>,
    /// A assessment specification can be composed of other "narrower"
    /// assessment specifications which when combined make up
    /// this assessment specification.
    has_part: Box<AssessmentSpecification>,
    /// An assessment specification (e.g. a standard) of which
    /// this specification is a specialisation.
    specialisation_of: Box<AssessmentSpecification>,
}

struct ScoringScheme;

/// The result of a process establishing the extent to which a learner
/// has attained particular knowledge, skills and competences against
/// criteria such as learning outcomes or standards of competence.
struct Assessment {
    /// A portable identifier of the assessment.
    id: URI,
    /// An alternative identifier assigned to
    /// the assessment by the organisation grading the assessment.
    identifier: Identifier,
    /// The title of the assessment.
    title: Text,
    /// The description of the assessment.
    description: Text,
    /// An additional free text note about the assessment.
    additional_note: Score,
    /// The resulting grade of the assessment.
    grade: Score,
    /// Indicator of how well the student was graded
    /// when compared to other students
    shortened_grading: ShortenedGrading,
    /// Describes a histogram of results achieved by
    /// all the students of a particular learning assessment.
    result_distribution: ResultDistribution,
    /// Date the grade was awarded.
    issued_date: chrono::naive::NaiveDateTime,
    /// Method of assessment supervision and id verification.
    id_verification: Code,
    /// The competent body that awarded the grade.
    assessed_by: Agent,
    /// The specification of this assessment.
    specified_by: AssessmentSpecification,
    /// Smaller assessments, which when combined make up
    /// and can influence this assessment.
    has_part: Box<Assessment>,
}

/// Indicator of how well the student was graded when compared
/// to other students.
struct ShortenedGrading {
    /// The percentage of students of the same
    /// course who got a lower grade.
    percentage_lower: Numeric,
    /// The percentage of students of the same
    /// course who got exactly the same grade.
    percentage_equal: Numeric,
    /// The percentage of students of the same
    /// course who got a higher grade.
    precentage_higher: Numeric,
}

/// Describes a histogram of results achieved by all the students
/// of this course instance.
struct ResultDistribution {
    /// Describes a single range within the histogram.
    category: ResultCategory,
    /// Free text description of the histogram.
    description: Note,
}

/// Description of a single score or score range within
/// a histogram of results.
struct ResultCategory {
    /// The label of the histogram score or score range.
    /// Should correspond to the grading scheme which have been used.
    /// E.g. 'C', or '20-30'.
    label: Text,
    /// N/A
    score: Score,
    /// N/A
    min_score: Score,
    /// N/A
    max_score: Score,
    /// N/A
    count: PositiveInteger,
}

/// A set of criteria that measures varying levels of achievement.
struct GradingScheme {
    /// A portable and unique identifier of the Grading Scheme.
    id: URI,
    /// An alternative identifier of the Grading Scheme
    /// assigned to it by the organisation administering the scheme.
    identifier: Identifier,
    /// The title of the scoring scheme.
    title: Text,
    /// A free text description of the scoring scheme.
    description: Note,
    /// A public web document containing additional
    /// documentation about the scoring system.
    supplementary_document: WebDocument,
}

/// The acquisition of knowledge, skills or responsibility and autonomy.
/// A recognised and/or awarded set of learning outcomes of an individual.
struct LearningAchievement {
    /// A portable and identifier of the learning achievement.
    id: URI,
    /// An alternative identifier assigned to the achievement
    /// by the organisation awarding the achievement.
    identifier: Identifier,
    /// The title of the achievement.
    title: Text,
    /// A description of the achievement.
    description: Note,
    /// An additional free text note about the achievement.
    additional_note: Note,
    /// An assessment which proves the acquisition of
    /// the learning outcomes which make up the achievement.
    was_derived_from: Assessment,
    /// Activities which contributed to the acquisition of
    /// the learning outcomes which make up the achievement.
    was_influenced_by: LearningActivity,
    /// The awarding details of this achievement.
    was_awarded_by: AwardingProcess,
    /// Smaller units of achievement,
    /// which when combined make up this achievement.
    has_part: Box<LearningAchievement>,
    /// Entitlements the owner has received as a result of this achievement.
    entitles_to: Entitlement,
    /// What has been learned.
    specfied_by: LearningSpecification,
    /// The learning opportunity that was taken to obtain
    /// the awarded LearningSpecification.
    associated_learning_opportunity: LearningOpportunity,
}

/// A formal outcome of an assessment and validation process which is obtained
/// when a competent authority determines that an individual has achieved
/// learning outcomes to given standards.
struct QualificationAwarded {
    /// The details of the awarded qualification.
    specified_by: Qualification,
}

/// The process of an organisation awarding Learning Achievement to person based
/// on a Learning Specification (e.g. a qualification). It is used to specify
/// the organisation that awarded the LearningSpecification to the individual,
/// the country or region where the LearningSpecification was awarded,
/// and optionally the date of awarding.
struct AwardingProcess {
    /// A portable and Unique Identifier of the Awarding Process.
    id: URI,
    /// An alternative identifier of the awarding process.
    identifier: Identifier,
    /// A description of the awarding process to the individual.
    description: Text,
    /// An additional free text note
    /// (e.g. a comment, a remark, etc.)
    additional_note: Text,
    /// The assessment that provided the basis for this awarding.
    used: Assessment,
    /// The resulting learning achievement.
    learning_achievement: Box<LearningAchievement>,
    /// The awarding body that awarded the Achievement to the individual.
    /// Only in cases of co-awarding/co-graduation,
    /// where a qualification award is issued to an individual by two or
    /// more organisations the cardinality is greater than 1.
    awarding_body: Organisation,
    /// The location where the awarding activity took place
    /// (country/region where the qualification was awarded).
    awarding_location: Location,
    /// The date when the LearningSpecification was awarded.
    /// If not specified it is undefined (“not known”).
    awarding_date: chrono::naive::NaiveDateTime,
}

/// An awarding activity represents an activity related to the awarding of
/// a LearningSpecification. It is used to specify the country or region
/// where the LearningSpecification is awarded, the awarding body and
/// optionally the awarding period now or in the past.
struct AwardingOpportunity {
    /// A portable identifier of the awarding opportunity.
    id: URI,
    /// An alternative identifier of the awarding opportunity.
    identifier: Identifier,
    /// The awarding body related to this awarding activity,
    /// i.e the organisation that issues the qualification.
    /// Only in cases of co-awarding/co-graduation,
    /// where a qualification is issued to an individual by two
    /// or more organisations the cardinality is greater than 1.
    awarding_body: Organisation,
    /// Location where the awarding activity takes place,
    /// the country/region where the qualification is awarded.
    location: Code,
    /// The date since when the awarding activities take place.
    /// If not specified it is undefined (“not known”)
    started_at_time: chrono::naive::NaiveDateTime,
    /// The date until when the awarding activities take/took place
    ended_at_time: chrono::naive::NaiveDateTime,
}

/// A right, e.g. to practice a profession, take advantage of
/// a learning opportunity or join an organisation,
/// as a result of the acquisition of knowledge, skills,
/// responsibility and/or autonomy.
struct Entitlement {
    /// A portable and identifier of the entitlement.
    id: URI,
    /// An alternative identifier of the entitlement.
    identifier: Identifier,
    /// The title of the entitlement.
    title: Text,
    /// A free text description of the specific rights
    /// the holder of the credential has acquired.
    description: Note,
    /// The date from which the entitlement was conferred.
    issued_date: chrono::naive::NaiveDate,
    /// The date until which the entitlment was conferred.
    expiry_date: chrono::naive::NaiveDate,
    /// An additional free text note about the entitlement.
    additional_note: Note,
    /// A learning achievement which gave rise to the entitlement.
    specified_by: EntitlementSpecification,
    /// The learning achievement (and related learning outcomes)
    /// which gave rise to this entitlement.
    was_derived_from: Box<LearningAchievement>,
    /// Smaller entitlements, which when combined make up this entitlement.
    has_part: Box<Entitlement>,
}

/// The specification of a right a person has access to,
/// typically as a result of a learning achievement.
/// It may take the form of the right to be a member of an organisation,
/// to follow a certain learning opportunity specification,
/// or to follow a certain career.
struct EntitlementSpecification {
    /// A portable and unique identifier of the entitlement specification.
    id: URI,
    /// An alternative identifier of the entitlement specification.
    identifier: Identifier,
    /// The title of the entitlement specification.
    title: Text,
    /// An alternative name of the entitlement specification.
    alternative_label: Text,
    /// A free text description of the entitlement specification.
    description: Note,
    /// An additional free text note about the entitlement specification.
    additional_not: Note,
    /// The homepage (a public web document) of the entitlement specification.
    home_page: WebDocument,
    /// A public web document containing additional documentation about the entitlement specification.
    supplementary_document: WebDocument,
    /// A credential-holder may be entitled to membership of an organisation or professional association; to access a learning opportunity; or to perform a specific employment
    entitlement_type: controlled_vocabularies::EntitlementType,
    /// The status of the entitlement: an entitlement may be prospective, i.e. awarding the right to apply for the entitlement; or actual, i.e. granting the entitlement.
    status: controlled_vocabularies::EntitlementStatus,
    /// The organisation which acknowledges the entitlement (i.e. the organisation offering the learning opportunity, membership or employment opportunity)
    limit_organisation: Organisation,
    /// The jurisdiction for which the entitlement is valid (the region or country).
    limit_jurisdiction: Code,
    /// The An ESCO Occupation or Occupational class which the individual may access through the entitlement.
    limit_occupation: EscoOccupationAssociation,
    /// An Occupation or Occupational Category
    limit_national_occupation: OccupationAssociation,
    /// A learning specification this entitlement specification
    /// may be derived from.
    may_result_from: Box<LearningSpecification>,
    /// Smaller entitlement specifications, which when combined make up this entitlement specification.
    entitlement_specification: Box<EntitlementSpecification>,
    /// An entitlement specification (e.g. a standard) of which this specification is a specialization.
    specialization_of: Box<EntitlementSpecification>,
}

/// An enumeration of the official languages used in the European Union,
/// as of 2013-07-01. See https://eur-lex.europa.eu/eli/reg/1958/1(1)/2013-07-01
/// for details.
enum EuropeanLanguage {
    Bulgarian,
    Croatian,
    Czech,
    Danish,
    Dutch,
    English,
    Estonian,
    Finnish,
    French,
    German,
    Greek,
    Hungarian,
    Irish,
    Italian,
    Latvian,
    Lithuanian,
    Maltese,
    Polish,
    Portugese,
    Romanian,
    Slovak,
    Slovene,
    Spanish,
    Swedish
}

// ------- Media Classes -------
struct InteractiveWebResource;
struct Phone;
struct MailBox;
struct Address;
struct Location;
struct WebDocument;
struct MediaObject;
struct ImageObject;

// ------- Association Classes -------
struct AssociationObject;

/// A character string used to identify a resource.
/// An identifier is a character string used to uniquely identify
/// one instance of an object within an identification scheme that is managed by an agency.
/// The Identifier class is a critical aspect of the EDCI model.
/// It is used to identify persons and organisations.
/// In these cases and more, the identifier itself will be some sort of
/// alpha-numeric string but that string only has meaning if it is contextualised.
struct Identifier {
    /// Content string which is the identifier.
    /// A character string used to uniquely identify
    /// one instance of an object within an identification
    /// scheme that is managed by an agency.
    content: String,
    /// Identification of the identifier scheme.
    /// The identifier register (the managing/originating system of the identifier).
    /// This can be seen as the namespace in which the assigned identifier is unique.
    identifier_scheme_id: String,
    /// Identification of the version of the identifier scheme.
    identifier_scheme_version_id: String,
    /// Identification of the agent that manages the identifier scheme.
    /// The agent that issued the identifier. (e.g. a URI).
    identifier_scheme_agency_id: String,
    /// The name of the identifier scheme.
    identifier_scheme_name: String,
    /// The name of the agent that manages the identifier scheme.
    /// The agent that issued the identifier.
    identifier_scheme_agency_name: String,
    /// The date on which the identifier was issued.
    issued_date: chrono::naive::NaiveDate,
    /// A code used to classify the type of identifier.
    identifier_type: String,
}

/// A legal identifier is a formally issued identifier by a given
/// authorithy within a given jurisdiction.
/// The identifier has a spatial context.
struct LegalIdentifier {
    /// The identifier of the country and/or jurisdiction.
    /// Recommended RSA: MDR Countries Named Authority Lis. NUTS.
    spatial_id: String
}

/// A term from a controlled vocabulary. (a code from a code list)
/// Interoperability between data sets is increased dramatically when
/// terms from controlled vocabularies are used in favour of free text.
/// The conceptual Code data type is used wherever one or more controlled
/// vocabularies are known to exist for a particular domain of interest.
/// It is not the job of the JV/CV Vocabularies to mandate which controlled
/// vocabularies are used but we offer some guidance on how to use them.
struct Code {
    /// The term bieng described.
    target_notation: String,
    /// The identification of the controlled vocabulary
    /// (the code list). (e.g. a URI)
    target_framework_uri: String,
    /// The name of the controlled vocabulary (the code list).
    target_framework: String,
    /// The text equivalent of the code content component.
    target_name: String,
    /// A description of the target term.
    target_description: String,
    /// A portable identifier (i.e a URI) of the code.
    uri: String,
}

/// A character string (i.e. a finite set of characters)
/// generally in the form of words of a language.
struct Text {
    /// The content of the string.
    content: String,
    /// The language of the content field.
    language: EuropeanLanguage
}

/// A formatted character string (i.e. a finite set of characters)
/// generally in the form of words of a language. The character string
/// is passed/included in, and can be represented as, a (formatted) document
/// fragment (formatted) according a given mimetype (e.g. "text/plain", "text/html", etc.)
struct Note {
    /// The free text note.
    content: String,
    /// The identifier of the language used in the Content attribute.
    language: EuropeanLanguage,
    /// The identifier of the mimetype used in the Content attribute.
    format: String,
    /// The information topic this note is about.
    topic: String,
}

/// A notation (or code) is a character string according
/// a given syntax encoding scheme.
struct Notation {
    /// A notation, or its code.
    content: String,
    /// The syntax encoding scheme.
    /// A particular system of notations or classification codes.
    scheme_id: String,
}

/// A score associated with a credential.
struct Score {
    /// The score itself.
    content: f64,
    /// The identifier of the scoring scheme used in the Content attribute.
    /// Refers to the type of scoring methodology or convention.
    scoring_scheme: String,
}

/// A numeric score. Extends Score.
struct NumericScore {
    content: f64,

}

/// A textual accreditation. Extends Score.
struct TextualScore {
    content: String,
}

/// A standard measure with a unit from
/// the MDR Measurement unit Named Authority List.
struct Measure {
    /// The numeric value (i.e. measure).
    content: f64,
    /// A code indicating the type of unit measure,
    /// such as “minutes”, “hours”, “meters”, etc.
    ///
    /// Based on MDR Measurement unit Named Authority List.
    unit: controlled_vocabularies::MDRunit,
}

/// A standard amount with a unit from
/// the MDR Currencies Named Authority List.
struct Amount {
    /// The numeric value (i.e. measure).
    content: f64,
    /// A code indicating the currency the content field is in.
    /// Based on MDR Currencies Named Authority List.
    unit: controlled_vocabularies::MDRcurrency,
}

// ------- Primitive Types -------

/// A Uniform Resource Identifier.
/// Has a range of xsd:anyURI.
struct URI(String);

///A boolean indicating true or false.
/// Has a range of xsd:boolean.
type IndicatorType = bool;

/// A rate, number or proportion per hundred.
/// Has a range of xsd:decimal.
struct PercentType(f64);

/// A positive integer.
/// Has a range of xsd:positiveInteger.
struct PositiveInteger(u64);

/// A numeric value.
/// Has a range of xsd:decimal.
struct Numeric(f64);

/// A time duration.
/// Has a range of xsd:duration.
struct Duration(u64);

// ------- Additional Types Not In Spec -------

/// An associated field of education from another
/// semantic framework than the ISCED classification.
struct EducationSubjectAssociation;

/// An associated field of education from another
/// semantic framework than the ISCED classification.
struct EducationLevelAssociation;

/// Uknown.
struct EscoOccupationAssociation;

/// Unknown.
struct OccupationAssociation;
