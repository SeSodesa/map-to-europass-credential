/*!
This submodule provides a set of national qualification framework (NQF) enums,
that list the different national qualification levels acknowledged by
the European Union. They should be accessed through the top level
enum `NQF`, at least when used as struct members.

See https://www.oph.fi/en/education-and-qualifications/qualifications-frameworks
for a list of links to relevant documents and details.
*/

/// The root of the shallow national qualification framework tree.
/// Lists the nations that have provided their NQFs for review,
/// and provides access to them, when used as a member of a struct.
enum NQF {
    Austria(AustriaNQF),
    BosniaAndHerzegovina(BosniaAndHerzegovinaNQF),
    Bulgaria(BulgariaNQF),
    Croatia(CroatiaNQF),
    Cyprus(CyprusNQF),
    CzechRepublic(CzechRepublicNQF),
    Denmark(DenmarkNQF),
    Germany(GermanyNQF),
    Estonia(EstoniaNQF),
    Finland(FinlandNQF),
    France(FranceNQF),
    Greece(GreeceNQF),
    Hungary(HungaryNQF),
    Iceland(IcelandNQF),
    Ireland(IrelandNQF),
    Italy(ItalyNQF),
    Latvia(LatviaNQF),
    Liechtenstein(LiechtensteinNQF),
    Lithuania(LithuaniaNQF),
    Luxembourg(LuxembourgNQF),
    NorthMacedonia(NorthMacedoniaNQF),
    Malta(MaltaNQF),
    Montenegro(MontenegroNQF),
    Netherlands(NetherlandsNQF),
    Norway(NorwayNQF),
    Poland(PolandNQF),
    Portugal(PortugalNQF),
    Romania(RomaniaNQF),
    Serbia(SerbiaNQF),
    Slovakia(SlovakiaNQF),
    Slovenia(SloveniaNQF),
    Sweden(SwedenNQF),
    Switzerland(SwitzerlandNQF),
    Turkey(TurkeyNQF),
}

enum AustriaNQF {}
enum BosniaAndHerzegovinaNQF {}
enum BulgariaNQF {}
enum CroatiaNQF {}
enum CyprusNQF {}
enum CzechRepublicNQF {}
enum DenmarkNQF {}
enum GermanyNQF {}
enum EstoniaNQF {}

/// The national qualification framework levels of Finland.
/// See https://www.oph.fi/en/education-and-qualifications/qualifications-frameworks
enum FinlandNQF {
    /// Basic education syllabus and Preparatory education
    /// for working life and independent living (TELMA).
    Level2,
    /// Preparatory studies for general upper secondary school (LUVA),
    /// Preparatory education for vocational training (VALMA) and
    /// an Advanced syllabus for basic education in the arts.
    Level3,
    /// General upper secondary school syllabus and the Matriculation Examination,
    /// Upper secondary vocational qualifications and further vocational qualifications,
    /// Basic Examination in Prison Services, Fire Fighter Qualification,
    /// and Emergency Response Centre Operator Qualification, a Basic course for
    /// Border Guards and a Basic study module for non-commissioned officers.
    Level4,
    /// Specialist vocational qualifications, the Sub-Officer Qualification
    /// (Fire and Rescue Services), the Vocational Qualification in Air Traffic Control,
    /// a General level (1 and 2) study module for non-commissioned officers and
    /// a Master level study module for non-commissioned officers.
    Level5,
    /// Bachelor’s degrees at universities of applied sciences and Bachelor’s degrees
    /// at universities, Professional specialisation programmes provided by universities
    /// and universities of applied sciences intended for holders of a Bachelor's degrees
    /// or a UAS Bachelor’s degree, and Specialised training and Pastoral qualification
    /// provided by the church.
    Level6,
    /// Master’s degrees at universities of applied sciences and Master’s degrees at
    /// universities, Professional specialisation programmes provided by universities
    /// and universities of applied sciences intended for holders of a Master’s degree
    /// or a UAS Master’s degree, Advanced pastoral qualification, Senior staff officer
    /// course, Further studies in war economy and technology and Specific qualification
    /// on prescribing medicines.
    Level7,
    /// Universities’ and National Defence University scientific and artistic postgraduate
    /// degrees (licentiate and doctor degrees), the General Staff Officer’s Degree,
    /// the Specialist Degree in Veterinary Medicine, and Specialist training in medicine
    /// and Specialist training in dentistry.
    Level8,
}

enum FranceNQF {}
enum GreeceNQF {}
enum HungaryNQF {}
enum IcelandNQF {}
enum IrelandNQF {}
enum ItalyNQF {}
enum LatviaNQF {}
enum LiechtensteinNQF {}
enum LithuaniaNQF {}
enum LuxembourgNQF {}
enum NorthMacedoniaNQF {}
enum MaltaNQF {}
enum MontenegroNQF {}
enum NetherlandsNQF {}
enum NorwayNQF {}
enum PolandNQF {}
enum PortugalNQF {}
enum RomaniaNQF {}
enum SerbiaNQF {}
enum SlovakiaNQF {}
enum SloveniaNQF {}
enum SwedenNQF {}
enum SwitzerlandNQF {}
enum TurkeyNQF {}
