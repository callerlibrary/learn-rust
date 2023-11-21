<template>
  <div>
    <el-form :model="form" label-width="120px">
      <el-form-item label="JSON">
        <el-input type="textarea" autosize v-model="form.json" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="onSubmit">Submit</el-button>
        <el-button>Cancel</el-button>
      </el-form-item>
    </el-form>
    <el-form label-width="120px">
      <el-form-item label="RESULT">
        <el-input type="textarea" autosize disabled v-model="result" />
      </el-form-item>
    </el-form>
    
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
const form = ref({ json: '' })
// 结果
const result = ref('')

const onSubmit = () => {
  console.log('submit!', form.value)
  // TODO 生成序列化后的结果
}
</script>

<style scoped></style>
