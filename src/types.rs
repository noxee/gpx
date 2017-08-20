//! generic types for GPX

use geo::{ToGeo, Geometry};
use geo::{Point, LineString, MultiLineString};

use chrono::DateTime;
use chrono::prelude::Utc;

/// Gpx is the root element in the XML file.
#[derive(Default, Debug)]
pub struct Gpx {
    pub version: String,

    /// Metadata about the file.
    pub metadata: Option<Metadata>,

    /// A list of tracks.
    pub tracks: Vec<Track>,
}


/// Metadata is information about the GPX file, author, and copyright restrictions.
///
/// Providing rich, meaningful information about your GPX files allows others to
/// search for and use your GPS data.
#[derive(Default, Debug)]
pub struct Metadata {
    /// The name of the GPX file.
    pub name: Option<String>,

    /// A description of the contents of the GPX file.
    pub description: Option<String>,

    /// The person or organization who created the GPX file.
    pub author: Option<Person>,

    /// URLs associated with the location described in the file.
    pub links: Vec<Link>,

    /// The creation date of the file.
    pub time: Option<DateTime<Utc>>,

    /// Keywords associated with the file. Search engines or databases can use
    /// this information to classify the data.
    pub keywords: Option<String>,

    /*copyright: GpxCopyrightType,*/
    /*pub bounds: Option<Bbox<f64>>,*/
    /*extensions: GpxExtensionsType,*/
}


/// Track represents an ordered list of points describing a path.
#[derive(Default, Debug)]
pub struct Track {
    /// GPS name of track.
    pub name: Option<String>,

    /// GPS comment for track.
    pub comment: Option<String>,

    /// User description of track.
    pub description: Option<String>,

    /// Source of data. Included to give user some idea of reliability
    /// and accuracy of data.
    pub source: Option<String>,

    /// Links to external information about the track.
    pub links: Vec<Link>,

    /// Type (classification) of track.
    pub _type: Option<String>,

    /// A Track Segment holds a list of Track Points which are logically
    /// connected in order. To represent a single GPS track where GPS reception
    /// was lost, or the GPS receiver was turned off, start a new Track Segment
    /// for each continuous span of track data.
    pub segments: Vec<TrackSegment>,

    /* pub number: u8,*/
    /* extensions */
    /* trkSeg */
}

impl Track {
    /// Gives the multi-linestring that this track represents, which is multiple
    /// linestrings.
    pub fn multilinestring(&self) -> MultiLineString<f64> {
        self.segments.iter().map(|seg| seg.linestring()).collect()
    }
}

impl ToGeo<f64> for Track {
    fn to_geo(&self) -> Geometry<f64> {
        Geometry::MultiLineString(self.multilinestring())
    }
}


/// TrackSegment represents a list of track points.
///
/// This TrackSegment holds a list of Track Points which are logically
/// connected in order. To represent a single GPS track where GPS reception
/// was lost, or the GPS receiver was turned off, start a new Track Segment
/// for each continuous span of track data.
#[derive(Default, Debug)]
pub struct TrackSegment {
    /// Each Waypoint holds the coordinates, elevation, timestamp, and metadata
    /// for a single point in a track.
    pub points: Vec<Waypoint>,
    /* extensions */
}

impl TrackSegment {
    /// Gives the linestring of the segment's points, the sequence of points that
    /// comprises the track segment.
    pub fn linestring(&self) -> LineString<f64> {
        self.points.iter().map(|wpt| wpt.point()).collect()
    }
}

impl ToGeo<f64> for TrackSegment {
    fn to_geo(&self) -> Geometry<f64> {
        Geometry::LineString(self.linestring())
    }
}


/// Waypoint represents a waypoint, point of interest, or named feature on a
/// map.
#[derive(Default, Debug)]
pub struct Waypoint {
    // TODO: make private.
    pub point: Option<Point<f64>>,

    /// Elevation (in meters) of the point.
    pub elevation: Option<f64>,

    /// Creation/modification timestamp for element. Date and time in are in
    /// Univeral Coordinated Time (UTC), not local time! Conforms to ISO 8601
    /// specification for date/time representation. Fractional seconds are
    /// allowed for millisecond timing in tracklogs.
    pub time: Option<DateTime<Utc>>,

    /// The GPS name of the waypoint. This field will be transferred to and
    /// from the GPS. GPX does not place restrictions on the length of this
    /// field or the characters contained in it. It is up to the receiving
    /// application to validate the field before sending it to the GPS.
    pub name: Option<String>,

    /// GPS waypoint comment. Sent to GPS as comment.
    pub comment: Option<String>,

    /// A text description of the element. Holds additional information about
    /// the element intended for the user, not the GPS.
    pub description: Option<String>,

    /// Source of data. Included to give user some idea of reliability and
    /// accuracy of data. "Garmin eTrex", "USGS quad Boston North", e.g.
    pub source: Option<String>,

    /// Links to additional information about the waypoint.
    pub links: Vec<Link>,

    /// Text of GPS symbol name. For interchange with other programs, use the
    /// exact spelling of the symbol as displayed on the GPS. If the GPS
    /// abbreviates words, spell them out.
    pub symbol: Option<String>,

    /// Type (classification) of the waypoint.
    pub _type: Option<String>,

    // <magvar> degreesType </magvar> [0..1] ?
    // <geoidheight> xsd:decimal </geoidheight> [0..1] ?
    // <fix> fixType </fix> [0..1] ?
    // <sat> xsd:nonNegativeInteger </sat> [0..1] ?
    // <hdop> xsd:decimal </hdop> [0..1] ?
    // <vdop> xsd:decimal </vdop> [0..1] ?
    // <pdop> xsd:decimal </pdop> [0..1] ?
    // <ageofdgpsdata> xsd:decimal </ageofdgpsdata> [0..1] ?
    // <dgpsid> dgpsStationType </dgpsid> [0..1] ?
    // <extensions> extensionsType </extensions> [0..1] ?
}

impl Waypoint {
    /// Gives the geographical point of the waypoint.
    pub fn point(&self) -> Point<f64> {
        self.point.unwrap()
    }
}

impl ToGeo<f64> for Waypoint {
    fn to_geo(&self) -> Geometry<f64> {
        Geometry::Point(self.point())
    }
}


/// Person represents a person or organization.
#[derive(Default, Debug)]
pub struct Person {
    /// Name of person or organization.
    pub name: Option<String>,

    /// Email address.
    pub email: Option<String>,

    /// Link to Web site or other external information about person.
    pub link: Option<Link>,
}


/// Link represents a link to an external resource.
///
/// An external resource could be a web page, digital photo,
/// video clip, etc., with additional information.
#[derive(Default, Debug)]
pub struct Link {
    /// URL of hyperlink.
    pub href: String,

    /// Text of hyperlink.
    pub text: Option<String>,

    /// Mime type of content (image/jpeg)
    pub _type: Option<String>,
}