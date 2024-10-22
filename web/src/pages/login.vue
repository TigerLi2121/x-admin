<template>
  <div class="login-wrapper">
    <div class="login-container">
      <div class="title-container">
        <h1 class="title">X-Admin</h1>
        <t-form
          ref="form"
          class="item-container"
          :data="d"
          :rules="rules"
          label-width="0"
          @submit="onSubmit"
        >
          <t-form-item name="username">
            <t-input
              v-model="d.username"
              size="large"
              placeholder="请输入账号：admin"
            >
              <template #prefix-icon>
                <t-icon name="user" />
              </template>
            </t-input>
          </t-form-item>
          <t-form-item name="password">
            <t-input
              v-model="d.password"
              size="large"
              :type="showPsw ? 'text' : 'password'"
              clearable
              placeholder="请输入登录密码：admin"
            >
              <template #prefix-icon>
                <t-icon name="lock-on" />
              </template>
              <template #suffix-icon>
                <t-icon
                  :name="showPsw ? 'browse' : 'browse-off'"
                  @click="showPsw = !showPsw"
                />
              </template>
            </t-input>
          </t-form-item>
          <t-form-item class="btn-container">
            <t-button block size="large" type="submit"> 登录 </t-button>
          </t-form-item>
        </t-form>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from "vue";
import type { FormRule, SubmitContext } from "tdesign-vue-next";
import { MessagePlugin } from "tdesign-vue-next";
import { useRoute, useRouter } from "vue-router";
import { useUserStore } from "@/store";

const userStore = useUserStore();
const router = useRouter();
const route = useRoute();

const d = ref({ username: "admin", password: "admin" });
const showPsw = ref(false);

const rules: Record<string, FormRule[]> = {
  username: [{ required: true, message: "账号必填", type: "error" }],
  password: [{ required: true, message: "密码必填", type: "error" }],
};

const onSubmit = async (ctx: SubmitContext) => {
  if (ctx.validateResult === true) {
    try {
      const res = await userStore.login(d.value);
      if (res.code === 0) {
        MessagePlugin.success("登陆成功");
        const redirect = route.query.redirect as string;
        const redirectUrl = redirect ? decodeURIComponent(redirect) : "/";
        console.log("redirectUrl:", redirectUrl);
        router.push(redirectUrl);
      }
    } catch (e) {
      console.log(e);
      MessagePlugin.error(e.message);
    }
  }
};
</script>

<style lang="less" scoped>
.login-wrapper {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-size: cover;
  background-position: 100%;
  position: relative;
}

.login-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  text-align: center;
}

.title-container {
  .title {
    font: var(--td-font-headline-large);
    color: var(--td-text-color-primary);
    margin-top: var(--td-comp-margin-xs);
  }

  .sub-title {
    margin-top: var(--td-comp-margin-xxl);
  }
}

.item-container {
  width: 400px;
  margin-top: var(--td-comp-margin-xxxxl);

  .btn-container {
    margin-top: var(--td-comp-margin-xxxxl);
  }
}

@media screen and (max-height: 700px) {
  .copyright {
    display: none;
  }
}
</style>
