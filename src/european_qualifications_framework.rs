/*!
This submodule provides a Rust implementation of the European Qualifications Framework.
The implementation is an enumeration of the 8 different levels of international qualification,
as recognized by the European Union and its partners.

See https://europa.eu/europass/en/european-qualifications-framework-eqf
for relevant information and links.
*/

/// An enumeration of the standard Europass EQF qualification levels.
/// Each of the 8 levels is defined by a set of descriptors indicating
/// the learning outcomes relevant to qualifications at that level
/// in any qualifications system.
///
/// # Knowledge
///
/// In the context of EQF, knowledge is described as theoretical and/or factual.
///
/// # Skills
///
/// In the context of EQF, skills are described as cognitive
/// (involving the use of logical, intuitive and creative thinking)
/// and practical (involving manual dexterity and the use of methods,
/// materials, tools and instruments).
///
/// # Responsibility and autonomy
///
/// In the context of the EQF responsibility and autonomy is described as
/// the ability of the learner to apply knowledge and skills autonomously
/// and with responsibility.
///
/// See https://europa.eu/europass/en/description-eight-eqf-levels
/// for details.
pub enum EQFLevel {
    /// Knowledge
    ///     Basic general knowledge
    ///
    /// Skills
    ///     Basic skills required to carry out simple tasks
    ///
    /// Responsibility and autonomy
    ///      Work or study under direct supervision in a structured context
    Level1,
    /// #Knowledge
    ///
    /// Basic factual knowledge of a field of work or study
    ///
    /// # Skills
    ///
    /// Basic cognitive and practical skills required to use relevant information
    /// in order to carry out tasks and to solve routine problems using simple rules and tools
    ///
    /// # Responsibility and autonomy
    ///
    /// Work or study under supervision with some autonomy
    Level2,
    /// # Knowledge
    ///
    /// Knowledge of facts, principles, processes and general concepts,
    /// in a field of work or study
    ///
    /// # Skills
    ///
    /// A range of cognitive and practical skills required to accomplish
    /// tasks and solve problems by selecting and applying basic methods,
    /// tools, materials and information
    ///
    /// # Responsibility and autonomy
    ///
    /// Take responsibility for completion of tasks in work or study;
    /// adapt own behaviour to circumstances in solving problems
    Level3,
    /// # Knowledge
    ///
    /// Factual and theoretical knowledge in broad contexts within
    /// a field of work or study
    ///
    /// # Skills
    ///
    /// A range of cognitive and practical skills required to generate
    /// solutions to specific problems in a field of work or study
    ///
    /// # Responsibility and autonomy
    ///
    /// Exercise self-management within the guidelines of work or
    /// study contexts that are usually predictable,
    /// but are subject to change; supervise the routine work of others,
    /// taking some responsibility for the evaluation and improvement of
    /// work or study activities
    Level4,
    /// # Knowledge
    ///
    /// Comprehensive, specialised, factual and theoretical knowledge within
    /// a field of work or study and an awareness of the boundaries of that knowledge
    ///
    /// # Skills
    ///
    /// A comprehensive range of cognitive and practical skills required
    /// to develop creative solutions to abstract problems
    ///
    /// # Responsibility and autonomy
    ///
    /// Exercise management and supervision in contexts of work or
    /// study activities where there is unpredictable change; review and
    /// develop performance of self and others
    Level5,
    /// # Knowledge
    ///
    /// Advanced knowledge of a field of work or study,
    /// involving a critical understanding of theories and principles
    ///
    /// # Skills
    ///
    /// A comprehensive range of cognitive and practical skills required to
    /// develop creative solutions to abstract problems
    ///
    /// # Responsibility and autonomy
    ///
    /// Manage complex technical or professional activities or projects,
    /// taking responsibility for decision-making in unpredictable work or
    /// study contexts; take responsibility for managing professional development
    /// of individuals and groups
    Level6,
    /// # Knowledge
    /// Highly specialised knowledge, some of which is at the forefront
    /// of knowledge in a field of work or study, as the basis for original
    /// thinking and/or research
    ///
    /// Critical awareness of knowledge issues in a field and at the interface
    /// between different fields
    ///
    /// # Skills
    ///
    /// Specialised problem-solving skills required in research and/or innovation
    /// in order to develop new knowledge and procedures and to integrate knowledge
    /// from different fields
    ///
    /// # Responsibility and autonomy
    /// Manage and transform work or study contexts that are complex,
    /// unpredictable and require new strategic approaches;
    /// take responsibility for contributing to professional knowledge
    /// and practice and/or for reviewing the strategic performance of teams
    Level7,
    /// # Knowledge
    /// Knowledge at the most advanced frontier of a field of work or study and
    /// at the interface between fields
    ///
    /// # Skills
    /// The most advanced and specialised skills and techniques,
    /// including synthesis and evaluation, required to solve critical problems
    /// in research and/or innovation and to extend and redefine existing knowledge
    /// or professional practice
    ///
    /// # Responsibility and autonomy
    /// Demonstrate substantial authority, innovation, autonomy, scholarly and
    /// professional integrity and sustained commitment to the development of
    /// new ideas or processes at the forefront of work or study contexts
    /// including research
    Level8,
}
