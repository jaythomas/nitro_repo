<template>
  <el-menu
    :aria-expanded="true"
    default-active="0"
    class="el-menu-demo"
    mode="horizontal"
  >
    <el-menu-item @click="tab = 0" index="0">General Settings</el-menu-item>
    <el-menu-item @click="tab = 1" index="1">Password</el-menu-item>
    <el-menu-item @click="tab = 2" index="2">Permissions</el-menu-item>
  </el-menu>
  <div v-if="tab == 0">
    <el-alert
      v-if="settingForm.error.length != 0"
      :title="settingForm.error"
      type="error"
      :closable="false"
    />
    <el-alert
      v-if="settingForm.success.length != 0"
      :title="settingForm.success"
      type="success"
      :closable="false"
    />
    <el-form label-position="top" :model="settingForm" label-width="120px">
      <el-form-item>
        <el-form-item label="Name">
          <el-input v-model="settingForm.name"></el-input>
        </el-form-item>
        <el-form-item label="Email">
          <el-input v-model="settingForm.email"></el-input>
        </el-form-item>
        <!--Yeah, I know. But please don't judge -->
        <el-button
          :disabled="settingButton()"
          type="primary"
          @click="onSettingSubmit"
          >Update Settings</el-button
        >
      </el-form-item>
    </el-form>
  </div>
  <div v-if="tab == 1">
    <el-alert
      v-if="settingForm.error.length != 0"
      :title="settingForm.error"
      type="error"
      :closable="false"
    />
    <el-form label-position="top" :model="password" label-width="120px">
      <el-form-item>
        <el-form-item label="Passowrd">
          <el-input
            v-model="this.password.password"
            placeholder="Please input password"
            show-password
          />
        </el-form-item>
        <el-form-item label="Confirm Password">
          <el-input
            v-model="password.confirm"
            placeholder="Please input password"
            show-password
          />
        </el-form-item>
        <!--Yeah, I know. But please don't judge -->
        <el-button
          :disabled="
            password.password.length == 0 ||
            password.password != password.confirm
          "
          type="primary"
          @click="onSettingSubmit"
          >Update Passwords</el-button
        >
      </el-form-item>
    </el-form>
  </div>
  <div v-if="tab == 2">
    <el-alert
      v-if="permissions.error.length != 0"
      :title="permissions.error"
      type="error"
      closable="false"
    />
    <el-form label-position="top" :model="permissions" label-width="120px">
      <el-form-item>
        <el-form-item label="Admin">
          <el-switch v-model="permissions.admin" />
        </el-form-item>
        <el-form-item label="Deployer">
          <el-switch v-model="permissions.deployer" />
        </el-form-item>
        <!--Yeah, I know. But please don't judge -->
        <el-button :disabled="me" type="primary" @click="onSettingSubmit"
          >Update Permissions</el-button
        >
      </el-form-item>
    </el-form>
  </div>
</template>

<script lang="ts">
import axios from "axios";
import {
  AuthToken,
  BasicResponse,
  RepoSettings,
  Repository,
  DEFAULT_STORAGE,
  Storage,
  User,
} from "@/backend/Response";
import router from "@/router";
import http from "@/http-common";
import { computed, defineComponent, onMounted, ref } from "vue";
import { useCookie } from "vue-cookie-next";
import { useRouter } from "vue-router";
import { getStorage } from "@/backend/api/Storages";

export default defineComponent({
  props: {
    user: {
      required: true,
      type: Object as () => User,
    },
    me: {
      required: true,
      type: Boolean,
    },
  },

  setup(props) {
    let settingForm = ref({
      email: props.user.email,
      name: props.user.name,
      error: "",
      success: "",
    });
    let password = ref({
      password: "",
      confirm: "",
      error: "",
    });
    let permissions = ref({
      admin: props.user.permissions.admin,
      deployer: props.user.permissions.deployer,
      error: "",
    });

    const tab = ref(0);

    return { settingForm, password, tab, permissions };
  },
  methods: {
    settingButton() {
      return (
        this.$props.me ||
        (this.$props.user.name == this.settingForm.name &&
          this.$props.user.email == this.settingForm.email)
      );
    },
    async onSettingSubmit() {
      let newUser = {
        email: this.settingForm.email,
        name: this.settingForm.name,
      };
      let body = JSON.stringify(newUser);
      console.log(body);
      const res = await http.post(
        "/api/admin/user/" + this.$props.user.username + "/modify",
        body,
        {
          headers: {
            "Content-Type": "application/json",
            Authorization: "Bearer " + this.$cookie.getCookie("token"),
          },
        }
      );
      if (res.status != 200) {
        console.log("Data" + res.data);
        return;
      }
      const result = res.data;
      let value = JSON.stringify(result);
      console.log(value);

      let response: BasicResponse<unknown> = JSON.parse(value);

      if (response.success) {
        this.$notify({
          title: "Updated User",
          type:"success"
        });
      } else {
        this.settingForm.error = "Unable to Update user";
        console.log(response);
      }
    },
    async updatePassword() {
      let newUser = {
        password: this.password.password,
        password_two: this.password.confirm,
      };
      let body = JSON.stringify(newUser);
      console.log(body);
      const res = await http.post(
        this.$props.me
          ? "/api/admin/user/password"
          : "/api/admin/user/" + this.$props.user.username + "/password",
        body,
        {
          headers: {
            "Content-Type": "application/json",
            Authorization: "Bearer " + this.$cookie.getCookie("token"),
          },
        }
      );
      if (res.status != 200) {
        console.log("Data" + res.data);
        return;
      }
      const result = res.data;
      let value = JSON.stringify(result);
      console.log(value);

      let response: BasicResponse<unknown> = JSON.parse(value);

      if (response.success) {
        this.settingForm.success = "Updated User";
      } else {
        this.settingForm.error = "Unable to Update user";
        console.log(response);
      }
    },
  },
});
</script>
<style scoped></style>
