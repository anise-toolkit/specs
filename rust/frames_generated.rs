// automatically generated by the FlatBuffers compiler, do not modify



use crate::common_generated::*;
use crate::ephemeris_generated::*;
use crate::time_generated::*;
use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod anise {

  use crate::common_generated::*;
  use crate::ephemeris_generated::*;
  use crate::time_generated::*;
  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};
#[allow(unused_imports, dead_code)]
pub mod frame {

  use crate::common_generated::*;
  use crate::ephemeris_generated::*;
  use crate::time_generated::*;
  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

pub enum FrameOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Frame<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Frame<'a> {
    type Inner = Frame<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> Frame<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Frame { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args FrameArgs<'args>) -> flatbuffers::WIPOffset<Frame<'bldr>> {
      let mut builder = FrameBuilder::new(_fbb);
      if let Some(x) = args.children { builder.add_children(x); }
      if let Some(x) = args.constants { builder.add_constants(x); }
      if let Some(x) = args.ephemeris { builder.add_ephemeris(x); }
      builder.finish()
    }

    pub const VT_EPHEMERIS: flatbuffers::VOffsetT = 4;
    pub const VT_CONSTANTS: flatbuffers::VOffsetT = 6;
    pub const VT_CHILDREN: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn ephemeris(&self) -> Option<super::ephemeris::Ephemeris<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<super::ephemeris::Ephemeris>>(Frame::VT_EPHEMERIS, None)
  }
  #[inline]
  pub fn constants(&self) -> Option<super::common::ConstantMap<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<super::common::ConstantMap>>(Frame::VT_CONSTANTS, None)
  }
  #[inline]
  pub fn children(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Frame<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Frame>>>>(Frame::VT_CHILDREN, None)
  }
}

impl flatbuffers::Verifiable for Frame<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<super::ephemeris::Ephemeris>>(&"ephemeris", Self::VT_EPHEMERIS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<super::common::ConstantMap>>(&"constants", Self::VT_CONSTANTS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Frame>>>>(&"children", Self::VT_CHILDREN, false)?
     .finish();
    Ok(())
  }
}
pub struct FrameArgs<'a> {
    pub ephemeris: Option<flatbuffers::WIPOffset<super::ephemeris::Ephemeris<'a>>>,
    pub constants: Option<flatbuffers::WIPOffset<super::common::ConstantMap<'a>>>,
    pub children: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Frame<'a>>>>>,
}
impl<'a> Default for FrameArgs<'a> {
    #[inline]
    fn default() -> Self {
        FrameArgs {
            ephemeris: None,
            constants: None,
            children: None,
        }
    }
}
pub struct FrameBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> FrameBuilder<'a, 'b> {
  #[inline]
  pub fn add_ephemeris(&mut self, ephemeris: flatbuffers::WIPOffset<super::ephemeris::Ephemeris<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<super::ephemeris::Ephemeris>>(Frame::VT_EPHEMERIS, ephemeris);
  }
  #[inline]
  pub fn add_constants(&mut self, constants: flatbuffers::WIPOffset<super::common::ConstantMap<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<super::common::ConstantMap>>(Frame::VT_CONSTANTS, constants);
  }
  #[inline]
  pub fn add_children(&mut self, children: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Frame<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Frame::VT_CHILDREN, children);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> FrameBuilder<'a, 'b> {
    let start = _fbb.start_table();
    FrameBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Frame<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Frame<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Frame");
      ds.field("ephemeris", &self.ephemeris());
      ds.field("constants", &self.constants());
      ds.field("children", &self.children());
      ds.finish()
  }
}
}  // pub mod Frame
}  // pub mod Anise

