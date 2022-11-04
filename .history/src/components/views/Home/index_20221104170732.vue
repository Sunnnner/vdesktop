<template>
    <n-space vertical size="large">
      <n-layout>
        <n-layout-header>
            <n-layout-header-title>Header</n-layout-header-title>
        </n-layout-header>
        <n-layout has-sider>
            <n-layout has-sider>
                <n-layout-sider content-style="padding: 24px;">
            <n-layout-sider
                bordered
                collapse-mode="width"
                :collapsed-width="64"
                :width="240"
                :collapsed="collapsed"
                show-trigger
                @collapse="collapsed = true"
                @expand="collapsed = false"
            >
                <n-menu
                :collapsed="collapsed"
                :collapsed-width="64"
                :collapsed-icon-size="22"
                :options="menuOptions"
                :render-label="renderMenuLabel"
                :render-icon="renderMenuIcon"
                :expand-icon="expandIcon"
                />
          </n-layout-sider>
            </n-layout>
          
          <n-layout-content content-style="padding: 24px;">
            平山道
          </n-layout-content>
        </n-layout>
        <n-layout-footer>成府路</n-layout-footer>
      </n-layout>

    </n-space>
  </template>
  
<script lang="ts" setup>
import { MenuOption, NIcon } from 'naive-ui';
import { h, ref } from 'vue';
import { BookmarkOutline, CaretDownOutline } from '@vicons/ionicons5'


const collapsed =ref(true)
const menuOptions: MenuOption[] = [
  {
    label: '且听风吟',
    key: 'hear-the-wind-sing',
    href: 'https://baike.baidu.com/item/%E4%B8%94%E5%90%AC%E9%A3%8E%E5%90%9F/3199'
  },
  {
    label: '1973年的弹珠玩具',
    key: 'pinball-1973',
    disabled: true,
    children: [
      {
        label: '鼠',
        key: 'rat'
      }
    ]
  },
  {
    label: '寻羊冒险记',
    key: 'a-wild-sheep-chase',
    disabled: true
  },
  {
    label: '舞，舞，舞',
    key: 'dance-dance-dance',
    children: [
      {
        type: 'group',
        label: '人物',
        key: 'people',
        children: [
          {
            label: '叙事者',
            key: 'narrator'
          },
          {
            label: '羊男',
            key: 'sheep-man'
          }
        ]
      },
      {
        label: '饮品',
        key: 'beverage',
        children: [
          {
            label: '威士忌',
            key: 'whisky',
            href: 'https://baike.baidu.com/item/%E5%A8%81%E5%A3%AB%E5%BF%8C%E9%85%92/2959816?fromtitle=%E5%A8%81%E5%A3%AB%E5%BF%8C&fromid=573&fr=aladdin'
          }
        ]
      },
      {
        label: '食物',
        key: 'food',
        children: [
          {
            label: '三明治',
            key: 'sandwich'
          }
        ]
      },
      {
        label: '过去增多，未来减少',
        key: 'the-past-increases-the-future-recedes'
      }
    ]
  }
]

const renderMenuLabel = (option: MenuOption) =>{
    if ('href' in option) {
          return h(
            'a',
            { href: option.href, target: '_blank' },
            option.label as string
          )
        }
        return option.label as string
}

const renderMenuIcon  = (option: MenuOption) => {
    if (option.key === 'sheep-man') return true
        // 返回 falsy 值，不再渲染图标及占位符
    if (option.key === 'food') return null
        return h(NIcon, null, { default: () => h(BookmarkOutline) })
}

const expandIcon = () =>{
    return h(NIcon, null, { default: () => h(CaretDownOutline) })
}

</script>

  <style scoped>
  .n-layout-header,
  .n-layout-footer {
    background: rgba(128, 128, 128, 0.2);
    padding: 24px;
  }
  
  .n-layout-sider {
    background: rgba(128, 128, 128, 0.3);
  }
  
  .n-layout-content {
    background: rgba(128, 128, 128, 0.4);
  }
  </style>