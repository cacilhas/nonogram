ifdef USE_UPX
INSTALL= install
UPX= upx
else
INSTALL= install -s
endif

BUILD= go build
MD= mkdir -p
RM= rm -f
TESTER= go test -timeout 30s

BINDIR= $(GOPATH)/bin
TARGET= nonogram.x86_64
SOURCE= $(wildcard *.go nonogram/*.go)
TESTS= $(wildcard tests/*.go)


#-------------------------------------------------------------------------------
.PHONY: clean install mrproper uninstall test

all: $(TARGET)


$(TARGET): $(SOURCE)
	$(BUILD) -o $@ .


$(BINDIR):
	$(MD) $@


clean:
	$(RM) $(TARGET)


install: $(TARGET)
	$(INSTALL) $(TARGET) $(BINDIR)
ifdef USE_UPX
	$(UPX) $(BINDIR)/$(TARGET)
endif


uninstall:
	$(RM) $(BINDIR)/$(TARGET)


test: $(TESTS) $(SOURCE)
	$(TESTER) $(TESTS)
