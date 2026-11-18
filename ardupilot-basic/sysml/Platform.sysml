//@ HAMR: --platform Microkit --verus-attribute-syntax --sel4-output-dir ../hamr/microkit
package Platform {
  private import HAMR::*;
  private import SW::*;
  
  part def Proc :> Processor {
    attribute :>> Frame_Period = 180 [us];
    attribute :>> Clock_Period = 1 [us];
    attribute :>> Max_Domain = 6;
  }
  
  part def ZCU102_Impl :> System {
    part seL4 : SW::seL4;

    attribute :>> Scheduling = MCS;

  }

  part def ZCU102_Impl_MOCK :> ZCU102_Impl {
      part :>> seL4 : SW::seL4_MOCK;
  }
}
