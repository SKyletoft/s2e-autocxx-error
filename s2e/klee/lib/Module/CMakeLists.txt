#===------------------------------------------------------------------------===#
#
#                     The KLEE Symbolic Virtual Machine
#
# This file is distributed under the University of Illinois Open Source
# License. See LICENSE.TXT for details.
#
#===------------------------------------------------------------------------===#
klee_add_component(kleeModule
	InstructionOperandTypeCheckPass.cpp
	IntrinsicCleaner.cpp
	KInstruction.cpp
	KModule.cpp
	LowerSwitch.cpp
	ModuleUtil.cpp
	PhiCleaner.cpp
	RaiseAsm.cpp
)
