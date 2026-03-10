package Platform {
	private import CASE_Scheduling::*;
	private import HAMR::*;
	private import SW::*;
	
	part def Proc :> Processor {
		attribute :>> Frame_Period = 1850 [ms];
		attribute :>> Clock_Period = 2 [ms];
		attribute Max_Domain : CASE_Scheduling::Max_Domain = 6;
		attribute Schedule_Source_Text : ScalarValues::String = "domain_schedule.c";
	}
	
	part def ZCU102_Impl :> System {
		part proc : Proc;
		part seL4 : SW::seL4;
		
		allocation allo :  Deployment_Properties::Actual_Processor_Binding
			allocate seL4 to proc; // (reference (proc)) applies to seL4
	}
}
