//      A Frame is the minimum level of network transaction and it is only used in the networkstack.
//   First 96 bits in the frame is fixed length that tells information about remainings. Current 
//   definition only suitable for TCP connection.
//   
//
//   0                7                 15                23                 31
//   +------------------------------------------------------------------------+   0
//   |    version     |      ftype      |      flags      |        id         |
//   +------------------------------------------------------------------------+   32
//   |                id                |        frame size (in octans)       |
//   +------------------------------------------------------------------------+   64
//   |                               padding                                  |
//   +------------------------------------------------------------------------+   96
//   |                               payload                                  |
//   +------------------------------------------------------------------------+   ...
//
//   # version (8-bits)
//   Version is always set to 0x01.
//
//   # Frame Type (8-bits)
//   ```c
//   enum FRAME_TYPE {
//     TEXT = 1,
//     KEY_VALUE = 2,
//     RAW = 3,
//     PING = 4,
//   }
//   ```
//
//   Frame type tells it payload format, helps to reconstruce its header/body. The Frame Type
//   MUST NOT BE 0x00. If frame type is 0x00 it means connection error.
//
//   >  TEXT      :  UTF-8 String
//   >  KEY_VALUE :  http2 like kv
//   >  RAW       :  binary data
//   >  PING      :  Ping is a special type that assumed not contains payload, frame size will
//   >               discarded.
//
//
//   # Flags (8-bits)
//   first bit is set if the frame is the last part of data.
//
//   # Id (24-bits)
//   Frame identifier, used to reconstruct multiplexed frame just like http2. Zero(0x00) is 
//   special id that means data is not multiplexed.
//
//   # Frame Size ( 16-bits)
//   Frame size is a 2 octant unsigned integer of the payload size in OCTANS(or Byte). It MAYBE
//   0x00 in certain frame types.
//
//   # Padding (32-bit)
//   32bit padding after frame size, that MUSTBE 0x00.
//   
pub struct Frame {
  version : u8,
  ftype : u8,
  flags : u8,
  id : (u8,u8,u8),
  size : u16,
  data : Option<Box<[u8]>>
}
