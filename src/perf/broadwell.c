#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#include <sys/syscall.h>
#include <string.h>
#include <sys/ioctl.h>
#include <linux/perf_event.h>
#include <linux/hw_breakpoint.h>
#include <asm/unistd.h>
#include <errno.h>
#include <inttypes.h>
#include "broadwell.h"

static char buf[4096];

static uint64_t raw_counters[] = {/*0x108,0x149,*/
                                  0x1008,0x1049,
                                  0x11bc,0x12bc,0x14bc,0x18bc};
//static uint64_t fix_counters[] = {PERF_COUNT_HW_INSTRUCTIONS, PERF_COUNT_HW_REF_CPU_CYCLES};
static uint64_t fix_counters[] = {PERF_COUNT_HW_INSTRUCTIONS};
static char* raw_strings[] = {"cycles", "instructions",
  /*"dtlb_load_misses.miss_causes_a_walk", "dtlb_store_misses.miss_causes_a_walk",*/
  "dtlb_load_misses.walk_duration", "dtlb_store_misses.walk_duration",
  "page_walker_loads.dtlb_l1", "page_walker_loads.dtlb_l2", "page_walker_loads.dtlb_l3", "page_walker_loads.dtlb_memory"};

pa create_counters()
{
  uint64_t* ids = malloc(sizeof(raw_counters) + sizeof(uint64_t)*3);
  struct perf_event_attr pea;
  uint32_t size = sizeof(struct perf_event_attr);
  memset(&pea, 0, size);
  pea.type = PERF_TYPE_HARDWARE;
  pea.size = size;
  pea.config = PERF_COUNT_HW_CPU_CYCLES;
  pea.disabled = 1;
  pea.exclude_kernel = 0;
  pea.exclude_hv = 1;
  pea.read_format = PERF_FORMAT_GROUP | PERF_FORMAT_ID;
  int fd0 = syscall(__NR_perf_event_open, &pea, 0, -1, -1, 0);
  if(fd0 < 0) exit(-1);
  int fd = ioctl(fd0, PERF_EVENT_IOC_ID, &ids[0]);
  if(fd  < 0) exit(-2);
  for(int i = 0; i < sizeof(fix_counters)/sizeof(fix_counters[0]); i++)
  {
    memset(&pea, 0, size);
    pea.type = PERF_TYPE_HARDWARE;
    pea.size = size;
    pea.config = fix_counters[i];
    pea.disabled = 1;
    pea.exclude_kernel = 0;
    pea.exclude_hv = 1;
    pea.read_format = PERF_FORMAT_GROUP | PERF_FORMAT_ID;
    fd = syscall(__NR_perf_event_open, &pea, 0, -1, fd0, 0);
    if(fd < 0) exit(-1);
    fd = ioctl(fd, PERF_EVENT_IOC_ID, &ids[i + 1]);
    if(fd < 0) exit(-2);
  }
  for(int i = 0; i < sizeof(raw_counters)/sizeof(raw_counters[0]); i++)
  {
    memset(&pea, 0, size);
    pea.type = PERF_TYPE_RAW;
    pea.size = size;
    pea.config = raw_counters[i];
    pea.disabled = 1;
    pea.exclude_kernel = 0;
    pea.exclude_hv = 1;
    pea.read_format = PERF_FORMAT_GROUP | PERF_FORMAT_ID;
    fd = syscall(__NR_perf_event_open, &pea, 0, -1, fd0, 0);
    if(fd < 0) exit(-1);
    fd = ioctl(fd, PERF_EVENT_IOC_ID, &ids[i + 1 + sizeof(fix_counters)/sizeof(fix_counters[0])]);
    if(fd < 0) exit(-2);
  }
  pa p;
  p.fd0 = fd0;
  p.ids = ids;
  return p;
}
void reset_counters(pa pa0)
{
  int fd = ioctl(pa0.fd0, PERF_EVENT_IOC_RESET, PERF_IOC_FLAG_GROUP);
  if (fd < 0) exit(-10);
}
void start_counters(pa pa0)
{
  int fd = ioctl(pa0.fd0, PERF_EVENT_IOC_ENABLE, PERF_IOC_FLAG_GROUP);
  if (fd < 0) exit(-11);
}
void stop_counters(pa pa0)
{
  int fd = ioctl(pa0.fd0, PERF_EVENT_IOC_DISABLE, PERF_IOC_FLAG_GROUP);
  if (fd < 0) exit(-12);
}
void print_counters(pa pa0)
{

  uint64_t* vals = malloc(sizeof(raw_counters) + sizeof(uint64_t) + sizeof(fix_counters));
  rf* rf0 = (rf*) buf;
  int i = read(pa0.fd0, buf, sizeof(buf));
  if (i < 0) exit(-3);
  for(int i = 0; i < rf0->nr; i++)
  {
    for(int j = 0; j < sizeof(raw_counters)/sizeof(raw_counters[0]) + 1 + sizeof(fix_counters); j++)
    {
      if(rf0->values[i].id == pa0.ids[j])
      {
        vals[j] = rf0->values[i].value;
        break;
      }
    }
  }
  for(int i = 0; i < rf0->nr; i++)
    printf("%" PRIu64 "\t%s\n", vals[i], raw_strings[i]);
}
