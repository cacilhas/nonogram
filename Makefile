ifdef USE_UPX
INSTALL= install
UPX= upx
else
INSTALL= install -s
endif

BUILD= go build
MD= mkdir -p
RM= rm -rf
TESTER= go test -timeout 30s
ZIP= zip -r

ifeq ($(UNAME), Windows_NT)
PLATFORM= Windows
TARGET= nonogram.exe
else
PLATFORM= $(shell go env GOOS | sed 's/^./\U&/')
TARGET= nonogram.x86_64
endif

BINDIR= $(GOPATH)/bin
SOURCE= $(wildcard *.go nonogram/*.go)
TESTS= $(wildcard tests/*.go)
ZIPFILE= Nonogram-$(PLATFORM).zip


#-------------------------------------------------------------------------------
.PHONY: clean install itch mrproper uninstall test

all: $(TARGET)


itch: $(ZIPFILE)


$(TARGET): $(SOURCE)
	$(BUILD) -o $@ .


$(BINDIR):
	$(MD) $@


$(PLATFORM):
	$(MD) $@


$(ZIPFILE): $(PLATFORM) $(PLATFORM)/$(TARGET)
	$(ZIP) $@ $<


$(PLATFORM)/$(TARGET): $(TARGET) $(PLATFORM)
	$(INSTALL) $^
ifdef USE_UPX
	$(UPX) $@
endif


clean:
	$(RM) $(TARGET) $(PLATFORM) $(ZIPFILE)


install: $(TARGET)
	$(INSTALL) $(TARGET) $(BINDIR)
ifdef USE_UPX
	$(UPX) $(BINDIR)/$(TARGET)
endif


uninstall:
	$(RM) $(BINDIR)/$(TARGET)


test: $(TESTS) $(SOURCE)
	$(TESTER) $(TESTS)
