// @ HAMR: --platform Microkit --sel4-output-dir ../hamr/microkit

//@ HAMR: --platform Microkit --verus-attribute-syntax --sel4-output-dir ../hamr/microkit_mcs
//@ HAMR: --platform Microkit --verus-attribute-syntax --sel4-output-dir ../hamr/microkit_dev
// @ HAMR: --platform Microkit --verus-attribute-syntax --sel4-output-dir ../hamr/microkit_initial
package Platform {
  private import HAMR::*;
  private import SW::*;
  
  part def Proc :> Processor {
    attribute :>> Frame_Period = 100 [ms];
    attribute :>> Clock_Period = 2 [ms];
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
