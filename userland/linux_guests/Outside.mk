# Programs and data files built outside the tree, for an image that carries
# them beside the guests above. Included by Guests.mk.

# Programs built outside the tree, put in the image's Linux tree:
# LINUX_GUEST_PROGRAMS="lua=/path/to/lua gojq=/path/to/gojq" puts each at
# /bin/<name>, signed and proved like every guest above. At most ten, on the
# service and reply ids below in list order.
LINUX_GUEST_PROGRAMS ?=
LINUX_GUEST_PROGRAM_SERVICES := 5110 5112 5114 5116 5118 5120 5122 5124 5126 5128
LINUX_GUEST_PROGRAM_REPLIES := 5111 5113 5115 5117 5119 5121 5123 5125 5127 5129
$(if $(word 11,$(LINUX_GUEST_PROGRAMS)),$(error LINUX_GUEST_PROGRAMS names more than ten programs))
linux_guest_program_name = $(firstword $(subst =, ,$(word $(1),$(LINUX_GUEST_PROGRAMS))))
linux_guest_program_path = $(lastword $(subst =, ,$(word $(1),$(LINUX_GUEST_PROGRAMS))))
$(foreach i,$(shell seq 1 $(words $(LINUX_GUEST_PROGRAMS))),$(eval $(call LINUX_GUEST,$(call linux_guest_program_name,$(i)),$(word $(i),$(LINUX_GUEST_PROGRAM_SERVICES)),$(word $(i),$(LINUX_GUEST_PROGRAM_REPLIES)),$(call linux_guest_program_path,$(i)))))

# Data files from outside the tree, put in the image's Linux tree:
# LINUX_GUEST_FILES="/usr/local/lib/python312.zip=/path/to/python312.zip"
# puts each file at its path under /linux. Data, not programs: nothing here
# can be run, so nothing here needs a proof.
LINUX_GUEST_FILES ?=
LINUX_GUEST_STORE_DEPS += $(foreach f,$(LINUX_GUEST_FILES),$(lastword $(subst =, ,$(f))))
LINUX_GUEST_STORE_ENTRIES += $(foreach f,$(LINUX_GUEST_FILES),--entry /linux$(firstword $(subst =, ,$(f)))=$(lastword $(subst =, ,$(f))))
