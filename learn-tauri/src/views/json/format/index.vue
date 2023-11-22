<template>
  <div>
    <el-form :model="form" label-width="120px">
      <el-form-item label="JSON">
        <el-input type="textarea" autosize v-model="form.json" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="onFormat">格式化</el-button>
      </el-form-item>
    </el-form>
    <el-form label-width="120px">
      <el-form-item label="RESULT">
        <el-input type="textarea" autosize disabled v-model="result" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="onCopy">复制</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";
import { ElMessageBox } from 'element-plus'

// 表单
const form = ref({ json: '' })
// 结果
const result = ref('')

/**
 * @description 格式化
 */
const onFormat = async () => {
  const res = await invoke<{ Str: string, Bool?: never } | { Str?: never, Bool: false }>('serialization', { json: form.value.json })
  if (res?.Bool === false) {
    ElMessageBox({
      title: '错误',
      message: 'JSON格式错误',
      type: 'error'
    })
    return
  }
  result.value = res.Str
}

// TODO: 复制
const onCopy = ()=>{}
</script>

<style scoped></style>
