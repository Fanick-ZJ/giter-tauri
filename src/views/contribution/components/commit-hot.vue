<script setup lang="ts">
import { CommitStatistic, Repository, YMDStr } from '@/types';
import { computed, nextTick, onMounted, PropType, Ref, ref, toRaw, watch } from 'vue';
import _ from 'lodash'
import { getDaysOfMonth, getWeekNumber } from '@/utils/tool';
import { NPopover } from 'naive-ui';
import { upToDataElement } from '@/components/repo-home/util';
import { DayStat } from '../types';

defineOptions({
  name: 'CommitHot'
})

const props = defineProps({
  stats: {
   type: Object as PropType<{
    year: number;
    stats: DayStat[] 
   }[]>,
   required: true
  }
})
const currentYear =  ref<number>(props.stats.length > 0 ? props.stats[0].year : 0)

const yearRange = computed(() => {
  return props.stats!.map((stat) => {
    return stat.year
  }).sort()
})

const currentYearDayStat = computed(() => {
  let filted = props.stats.filter((stat) => {
    return stat.year === currentYear.value
  })
  if (filted.length === 0) {
    return [] 
  }
  return filted[0].stats
})

const getWeekAndWeekDay = (date: Date) => {
    let weekDay = date.getDay()
    // 如果是周日的话，需要将周数加1
    let week = getWeekNumber(date) + (weekDay == 0 ? 1 : 0)
    // 如果是十二月的最后几天所在的那一周，是下一年的第一周，要设置为52，为了显示在一张图里
    if (date.getMonth() == 11 && (week == 0 || week == 1)) {
      week = 52
    }
  return {
    week,
    weekDay 
  }
}

const clearWeekDayList = () => {
  weekDayList.value = _.times(7, () => _.times(53, () => undefined))
  // 将所有日期都设置为{date: 'xxxx-xx-xx', count: 0}
  let days = getDaysOfMonth(currentYear.value!)
  const date = new Date(`${currentYear.value}-01-01`)
  date.setHours(0, 0, 0, 0)
  for (let i = 0; i < 12; i++) {
    date.setDate(1)
    date.setMonth(i)
    for (let j = 0; j < days[i]; j++) {
      date.setDate(j + 1)
      const { week, weekDay } = getWeekAndWeekDay(date)
      weekDayList.value[weekDay][week] = {
        date: `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`,
        count: 0
      }
    }
  }
}
const weekDayList = ref<(DayStat | undefined)[][]>(_.times(7, () => _.times(53, () => undefined)))

watch(() => currentYearDayStat.value,  (newVal) => {
  clearWeekDayList()
  newVal.forEach((day: DayStat) => {
    let dateObj = new Date(day.date)
    const { week, weekDay } = getWeekAndWeekDay(dateObj)
    weekDayList.value[weekDay][week] = day
  })
}, {immediate: true})

const switchYear = (year: number) => {
  currentYear.value = year 
  emit('switch-year', year)
}

const months = [
  '一月',
  '二月',
  '三月',
  '四月',
  '五月',
  '六月',
  '七月',
  '八月',
  '九月',
  '十月',
  '十一月',
  '十二月'
]

const monthColspans = [5, 4, 4, 5, 4, 4, 5, 4, 4, 5, 4, 5]

const dayTitles = [
 '周一',
 '周三',
 '周五'
]

const max = ref(20)
const colorSteps = [
  '#ebedf0', 
  '#9be9a8',
  '#40c463',
  '#30a14e',
  '#216e39'
]

const pos = ref({
  x: 0,
  y: 0 
})
const showPopover = ref(false)
const popoverText = ref('')
const handleMove = _.debounce((e: MouseEvent) => {
  let target = e.target as HTMLElement
  const el = upToDataElement(target, 'data-contribution-popover')
  if (el) {
    const rect = el.getBoundingClientRect()
    pos.value = {
      x: rect.x + rect.width / 2,
      y: rect.y 
    }
    showPopover.value = true
    popoverText.value = el.getAttribute('data-contribution-popover')! 
  } else {
    showPopover.value = false 
  }
}, 100)

const handleClick = (e: MouseEvent) => {
  let target = e.target as HTMLElement
  const el = upToDataElement(target, 'data-date')
  if (el) {
    emit('date-click', el.getAttribute('data-date')!)
  } 
}

const emit = defineEmits({
  'switch-year': (year: number) => true ,
  'date-click': (date: string) => true,
})

</script>
<template>
  <div ref="containerRef"
    class="w-full flex h-[130px] gap-2">
    <div class="flex-1 overflow-x-auto overflow-y-hidden min-w-0 relative">
      <table
        @mousemove="handleMove"
        class="border-separate border-spacing-1 w-max absolute">
        <thead>
          <tr class="h-[13px]">
            <td class="w-[28px]"></td>
            <td :colspan="monthColspans[index]"
              class="relative text-xs leading-none"
              style="padding: .125em .5em .125em 0;" 
              v-for="(month, index) in months">
              <span class="absolute top-0">
                {{month}}
              </span>
            </td>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(days, i) in weekDayList">
            <td class="w-[28px] text-xs leading-[0] relative">
              <div class="absolute bottom-[-1em]">
                {{ i % 2 == 0 ? dayTitles[i / 2] : ''}}
              </div>
            </td>
            <td v-for="day in days" class="w-[10px] h-[10px]">
              <div v-if="day && day.count >= 0"
                class="w-full h-full rounded-sm"
                @click="handleClick"
                :data-date="day?.date"
                :data-contribution-popover="`${day?.date} 提交次数：${day?.count}`"
                :style="{
                  backgroundColor:day.count > 0 
                                ? colorSteps[Math.min(Math.ceil(day.count / max * colorSteps.length), colorSteps.length - 1)]
                               : '#ebedf0',
              }"></div>
            </td>
          </tr>
        </tbody>
      </table>
      <NPopover
        :show="showPopover"
        :x="pos.x"
        :y="pos.y"
        trigger="manual"
        class="text-xs"
        >
        {{ popoverText }}
      </NPopover>
    </div>
    <div class="flex flex-col gap-2 w-[100px]">
      <div v-for="year in yearRange">
        <div class="rounded-sm pl-1 pr-10 py-1" 
          :class="currentYear == year ? 'bg-sky-600 text-white' : 'hover:bg-slate-100 transition-all duration-50 ease-in-out'"
          @click="switchYear(year)">
          <span>{{year}}</span>
        </div>
      </div>
    </div>
  </div>
</template>


<style scoped>
</style>