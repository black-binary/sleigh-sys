#include "bridge.hh"
//#include "../target/cxxbridge/sleigh-sys/src/lib.rs.h"
#include "sleigh-sys/src/lib.rs.h"
#include <mutex>

unique_ptr<Decompiler> newDecompiler(RustLoadImage *loadImage,
                                     unique_ptr<DocumentStorage> spec) {
  auto l = unique_ptr<LoadImage>(new RustLoadImageProxy(loadImage));
  return make_unique<Decompiler>(move(l), move(spec));
}

unique_ptr<Address> newAddress() { return make_unique<Address>(); }

uint32_t getAddrSpaceType(const AddrSpace &space) {
  return (uint32_t)space.getType();
}

unique_ptr<Address> getVarnodeDataAddress(const VarnodeData &data) {
  return make_unique<Address>(data.getAddr());
}

unique_ptr<ContextDatabase> newContext() {
  return unique_ptr<ContextDatabase>(new ContextInternal());
}

unique_ptr<DocumentStorage> newDocumentStorage(const std::string &s) {
  static std::mutex lock;
  std::lock_guard<std::mutex> guard(lock);

  auto doc = make_unique<DocumentStorage>();
  std::stringstream ss;
  ss << s;
  auto root = doc->parseDocument(ss)->getRoot();
  doc->registerTag(root);
  return doc;
}

void RustLoadImageProxy::loadFill(uint1 *ptr, int4 size,
                                  const Address &address) {
  return inner->load_fill(ptr, size, address);
}

void RustLoadImageProxy::adjustVma(long adjust) {
  return inner->adjust_vma(adjust);
}

void RustPCodeEmitProxy::dump(const Address &addr, OpCode opc,
                              VarnodeData *outvar, VarnodeData *vars,
                              int4 isize) {
  inner->dump(addr, (uint32_t)opc, outvar, vars, isize);
}

int32_t Decompiler::translate(RustPCodeEmit *emit, uint64_t addr) const {
  auto address = Address(this->getDefaultCodeSpace(), addr);
  auto p = RustPCodeEmitProxy(emit);
  int32_t n = 0;
  try {
    n = this->oneInstruction(p, address);
  } catch (...) {
    // TODO
  }
  return n;
}

int32_t Decompiler::disassemble(RustAssemblyEmit *emit, uint64_t addr) const {
  auto address = Address(this->getDefaultCodeSpace(), addr);
  auto p = RustAssemblyEmitProxy(emit);
  int32_t n = 0;
  try {
    n = this->printAssembly(p, address);
  } catch (...) {
    // TODO
  }
  return n;
}

uint32_t getVarnodeSize(const VarnodeData &data) { return data.size; }

void RustAssemblyEmitProxy::dump(const Address &addr, const string &mnem,
                                 const string &body) {
  this->inner->dump(addr, mnem, body);
}