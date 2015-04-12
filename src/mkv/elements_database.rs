use mkv::ElementType::*;

macro_rules! elements_database {
    ($($cl:ident $t:ident $id:expr),*,) => {
        #[derive(Debug,Eq,PartialEq,Copy,Clone)]
        pub enum Class {
            $($cl),*
            ,Unknown
        }
        
        pub fn id_to_class(id:u64) -> Class {
            use self::Class::*;
            
            match id {
               $($id => $cl),*
               ,_ => Unknown,
            }
        }
        
        pub fn class_to_id(c:Class) -> u64 {
            use self::Class::*;
            
            match c {
                $($cl => $id),*
                ,Unknown => panic!("Trying to get ID of Unknown Matroska class"),
            }
        }
        
        pub fn class_to_type(c:Class) -> ::mkv::ElementType {
            use self::Class::*;
            
            match c {
                $($cl  => $t),*
                ,Unknown => Binary
            }
        }
    }
}

elements_database!(
/* Class                       Type           ID */
  EBML                        Master       0x1A45DFA3,
  EBMLVersion                 Unsigned     0x00004286,
  EBMLReadVersion             Unsigned     0x000042F7,
  EBMLMaxIDLength             Unsigned     0x000042F2,
  EBMLMaxSizeLength           Unsigned     0x000042F3,
  DocType                     TextAscii    0x00004282,
  DocTypeVersion              Unsigned     0x00004287,
  DocTypeReadVersion          Unsigned     0x00004285,
  Void                        Binary       0x000000EC,
  CRC32                       Binary       0x000000BF,
  SignatureSlot               Master       0x1B538667,
  SignatureAlgo               Unsigned     0x00007E8A,
  SignatureHash               Unsigned     0x00007E9A,
  SignaturePublicKey          Binary       0x00007EA5,
  Signature                   Binary       0x00007EB5,
  SignatureElements           Master       0x00007E5B,
  SignatureElementList        Master       0x00007E7B,
  SignedElement               Binary       0x00006532,
  Segment                     Master       0x18538067,
  SeekHead                    Master       0x114D9B74,
  Seek                        Master       0x00004DBB,
  SeekID                      Binary       0x000053AB,
  SeekPosition                Unsigned     0x000053AC,
  Info                        Master       0x1549A966,
  SegmentUID                  Binary       0x000073A4,
  SegmentFilename             TextUtf8     0x00007384,
  PrevUID                     Binary       0x003CB923,
  PrevFilename                TextUtf8     0x003C83AB,
  NextUID                     Binary       0x003EB923,
  NextFilename                TextUtf8     0x003E83BB,
  SegmentFamily               Binary       0x00004444,
  ChapterTranslate            Master       0x00006924,
  ChapterTranslateEditionUID  Unsigned     0x000069FC,
  ChapterTranslateCodec       Unsigned     0x000069BF,
  ChapterTranslateID          Binary       0x000069A5,
  TimecodeScale               Unsigned     0x002AD7B1,
  Duration                    Float        0x00004489,
  DateUTC                     Date         0x00004461,
  Title                       TextUtf8     0x00007BA9,
  MuxingApp                   TextUtf8     0x00004D80,
  WritingApp                  TextUtf8     0x00005741,
  Cluster                     Master       0x1F43B675,
  Timecode                    Unsigned     0x000000E7,
  SilentTracks                Master       0x00005854,
  SilentTrackNumber           Unsigned     0x000058D7,
  Position                    Unsigned     0x000000A7,
  PrevSize                    Unsigned     0x000000AB,
  SimpleBlock                 Binary       0x000000A3,
  BlockGroup                  Master       0x000000A0,
  Block                       Binary       0x000000A1,
  BlockVirtual                Binary       0x000000A2,
  BlockAdditions              Master       0x000075A1,
  BlockMore                   Master       0x000000A6,
  BlockAddID                  Unsigned     0x000000EE,
  BlockAdditional             Binary       0x000000A5,
  BlockDuration               Unsigned     0x0000009B,
  ReferencePriority           Unsigned     0x000000FA,
  ReferenceBlock              Signed       0x000000FB,
  ReferenceVirtual            Signed       0x000000FD,
  CodecState                  Binary       0x000000A4,
  Slices                      Master       0x0000008E,
  TimeSlice                   Master       0x000000E8,
  LaceNumber                  Unsigned     0x000000CC,
  FrameNumber                 Unsigned     0x000000CD,
  BlockAdditionID             Unsigned     0x000000CB,
  Delay                       Unsigned     0x000000CE,
  SliceDuration               Unsigned     0x000000CF,
  ReferenceFrame              Master       0x000000C8,
  ReferenceOffset             Unsigned     0x000000C9,
  ReferenceTimeCode           Unsigned     0x000000CA,
  EncryptedBlock              Binary       0x000000AF,
  Tracks                      Master       0x1654AE6B,
  TrackEntry                  Master       0x000000AE,
  TrackNumber                 Unsigned     0x000000D7,
  TrackUID                    Unsigned     0x000073C5,
  TrackType                   Unsigned     0x00000083,
  FlagEnabled                 Unsigned     0x000000B9,
  FlagDefault                 Unsigned     0x00000088,
  FlagForced                  Unsigned     0x000055AA,
  FlagLacing                  Unsigned     0x0000009C,
  MinCache                    Unsigned     0x00006DE7,
  MaxCache                    Unsigned     0x00006DF8,
  DefaultDuration             Unsigned     0x0023E383,
  TrackTimecodeScale          Float        0x0023314F,
  TrackOffset                 Signed       0x0000537F,
  MaxBlockAdditionID          Unsigned     0x000055EE,
  Name                        TextUtf8     0x0000536E,
  Language                    TextAscii    0x0022B59C,
  CodecID                     TextAscii    0x00000086,
  CodecPrivate                Binary       0x000063A2,
  CodecName                   TextUtf8     0x00258688,
  AttachmentLink              Unsigned     0x00007446,
  CodecSettings               TextUtf8     0x003A9697,
  CodecInfoURL                TextAscii    0x003B4040,
  CodecDownloadURL            TextAscii    0x0026B240,
  CodecDecodeAll              Unsigned     0x000000AA,
  TrackOverlay                Unsigned     0x00006FAB,
  TrackTranslate              Master       0x00006624,
  TrackTranslateEditionUID    Unsigned     0x000066FC,
  TrackTranslateCodec         Unsigned     0x000066BF,
  TrackTranslateTrackID       Binary       0x000066A5,
  Video                       Master       0x000000E0,
  FlagInterlaced              Unsigned     0x0000009A,
  StereoMode                  Unsigned     0x000053B8,
  OldStereoMode               Unsigned     0x000053B9,
  PixelWidth                  Unsigned     0x000000B0,
  PixelHeight                 Unsigned     0x000000BA,
  PixelCropBottom             Unsigned     0x000054AA,
  PixelCropTop                Unsigned     0x000054BB,
  PixelCropLeft               Unsigned     0x000054CC,
  PixelCropRight              Unsigned     0x000054DD,
  DisplayWidth                Unsigned     0x000054B0,
  DisplayHeight               Unsigned     0x000054BA,
  DisplayUnit                 Unsigned     0x000054B2,
  AspectRatioType             Unsigned     0x000054B3,
  ColourSpace                 Binary       0x002EB524,
  GammaValue                  Float        0x002FB523,
  FrameRate                   Float        0x002383E3,
  Audio                       Master       0x000000E1,
  SamplingFrequency           Float        0x000000B5,
  OutputSamplingFrequency     Float        0x000078B5,
  Channels                    Unsigned     0x0000009F,
  ChannelPositions            Binary       0x00007D7B,
  BitDepth                    Unsigned     0x00006264,
  TrackOperation              Master       0x000000E2,
  TrackCombinePlanes          Master       0x000000E3,
  TrackPlane                  Master       0x000000E4,
  TrackPlaneUID               Unsigned     0x000000E5,
  TrackPlaneType              Unsigned     0x000000E6,
  TrackJoinBlocks             Master       0x000000E9,
  TrackJoinUID                Unsigned     0x000000ED,
  TrickTrackUID               Unsigned     0x000000C0,
  TrickTrackSegmentUID        Binary       0x000000C1,
  TrickTrackFlag              Unsigned     0x000000C6,
  TrickMasterTrackUID         Unsigned     0x000000C7,
  TrickMasterTrackSegmentUID  Binary       0x000000C4,
  ContentEncodings            Master       0x00006D80,
  ContentEncoding             Master       0x00006240,
  ContentEncodingOrder        Unsigned     0x00005031,
  ContentEncodingScope        Unsigned     0x00005032,
  ContentEncodingType         Unsigned     0x00005033,
  ContentCompression          Master       0x00005034,
  ContentCompAlgo             Unsigned     0x00004254,
  ContentCompSettings         Binary       0x00004255,
  ContentEncryption           Master       0x00005035,
  ContentEncAlgo              Unsigned     0x000047E1,
  ContentEncKeyID             Binary       0x000047E2,
  ContentSignature            Binary       0x000047E3,
  ContentSigKeyID             Binary       0x000047E4,
  ContentSigAlgo              Unsigned     0x000047E5,
  ContentSigHashAlgo          Unsigned     0x000047E6,
  Cues                        Master       0x1C53BB6B,
  CuePoint                    Master       0x000000BB,
  CueTime                     Unsigned     0x000000B3,
  CueTrackPositions           Master       0x000000B7,
  CueTrack                    Unsigned     0x000000F7,
  CueClusterPosition          Unsigned     0x000000F1,
  CueBlockNumber              Unsigned     0x00005378,
  CueCodecState               Unsigned     0x000000EA,
  CueReference                Master       0x000000DB,
  CueRefTime                  Unsigned     0x00000096,
  CueRefCluster               Unsigned     0x00000097,
  CueRefNumber                Unsigned     0x0000535F,
  CueRefCodecState            Unsigned     0x000000EB,
  Attachments                 Master       0x1941A469,
  AttachedFile                Master       0x000061A7,
  FileDescription             TextUtf8     0x0000467E,
  FileName                    TextUtf8     0x0000466E,
  FileMimeType                TextAscii    0x00004660,
  FileData                    Binary       0x0000465C,
  FileUID                     Unsigned     0x000046AE,
  FileReferral                Binary       0x00004675,
  FileUsedStartTime           Unsigned     0x00004661,
  FileUsedEndTime             Unsigned     0x00004662,
  Chapters                    Master       0x1043A770,
  EditionEntry                Master       0x000045B9,
  EditionUID                  Unsigned     0x000045BC,
  EditionFlagHidden           Unsigned     0x000045BD,
  EditionFlagDefault          Unsigned     0x000045DB,
  EditionFlagOrdered          Unsigned     0x000045DD,
  ChapterAtom                 Master       0x000000B6,
  ChapterUID                  Unsigned     0x000073C4,
  ChapterTimeStart            Unsigned     0x00000091,
  ChapterTimeEnd              Unsigned     0x00000092,
  ChapterFlagHidden           Unsigned     0x00000098,
  ChapterFlagEnabled          Unsigned     0x00004598,
  ChapterSegmentUID           Binary       0x00006E67,
  ChapterSegmentEditionUID    Unsigned     0x00006EBC,
  ChapterPhysicalEquiv        Unsigned     0x000063C3,
  ChapterTrack                Master       0x0000008F,
  ChapterTrackNumber          Unsigned     0x00000089,
  ChapterDisplay              Master       0x00000080,
  ChapString                  TextUtf8     0x00000085,
  ChapLanguage                TextAscii    0x0000437C,
  ChapCountry                 TextAscii    0x0000437E,
  ChapProcess                 Master       0x00006944,
  ChapProcessCodecID          Unsigned     0x00006955,
  ChapProcessPrivate          Binary       0x0000450D,
  ChapProcessCommand          Master       0x00006911,
  ChapProcessTime             Unsigned     0x00006922,
  ChapProcessData             Binary       0x00006933,
  Tags                        Master       0x1254C367,
  Tag                         Master       0x00007373,
  Targets                     Master       0x000063C0,
  TargetTypeValue             Unsigned     0x000068CA,
  TargetType                  TextAscii    0x000063CA,
  TagTrackUID                 Unsigned     0x000063C5,
  TagEditionUID               Unsigned     0x000063C9,
  TagChapterUID               Unsigned     0x000063C4,
  TagAttachmentUID            Unsigned     0x000063C6,
  SimpleTag                   Master       0x000067C8,
  TagName                     TextUtf8     0x000045A3,
  TagLanguage                 TextAscii    0x0000447A,
  TagDefault                  Unsigned     0x00004484,
  TagString                   TextUtf8     0x00004487,
  TagBinary                   Binary       0x00004485,
);


#[test]
fn eldb_test__id_to_class() {
    assert_eq!(id_to_class(0x1A45DFA3), Class::EBML);
    assert_eq!(id_to_class(0x002EB524), Class::ColourSpace);
    assert_eq!(id_to_class(0x000000EC), Class::Void);
    assert_eq!(id_to_class(0x00004285), Class::DocTypeReadVersion);
    assert_eq!(id_to_class(0x1A45DBBBB), Class::Unknown);
}

#[test]
fn eldb_test__class_to_id() {
    assert_eq!(class_to_id(Class::EBML              ), 0x1A45DFA3);
    assert_eq!(class_to_id(Class::ColourSpace       ), 0x002EB524);
    assert_eq!(class_to_id(Class::Void              ), 0x000000EC);
    assert_eq!(class_to_id(Class::DocTypeReadVersion), 0x00004285);
}

#[test]
fn eldb_test__class_to_type() {
    use ::mkv::ElementType::*;
    assert_eq!(class_to_type(Class::EBML              ), Master);
    assert_eq!(class_to_type(Class::ColourSpace       ), Binary);
    assert_eq!(class_to_type(Class::Void              ), Binary);
    assert_eq!(class_to_type(Class::DocTypeReadVersion), Unsigned);
}
