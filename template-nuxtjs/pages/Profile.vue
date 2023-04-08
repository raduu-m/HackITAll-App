<template>
  <div>
    <v-container>
      <v-card class="mx-auto" max-width="400" @click="activate">
        <!--Create a card with the QR code-->
        <v-card class="mx-auto" max-width="400" @click="activate">
          <v-card-title class="text-h5">{{ name }}</v-card-title>
          <v-img>
            <!--Center the canvas-->
            <div class="d-flex justify-center">
              <canvas ref="canvas" class="canvas"></canvas>
            </div>
          </v-img>
        </v-card>
        <v-card-text>
          <!--Disable the underline for text field-->
          <v-text-field readonly label="Name" v-model="name"></v-text-field>
          <v-text-field readonly label="Account ID" v-model="accountId"></v-text-field>
          <v-text-field readonly label="Balance (LEI)" v-model="balance"></v-text-field>
        </v-card-text>
      </v-card>
    </v-container>
    <!-- Make a overlay that contains the canvas with the QRCOde-->
    <v-overlay v-if="active" :value="true">
      <v-card class="mx-auto text-center" style="width: 400px; height: 200px;">
        <v-card-title class="text-h5 text-center">ACCOUNT ID</v-card-title>

        <!--Add the Account ID-->
        <v-card-text class="text-h1">
          @{{ accountId }}
        </v-card-text>
        <v-card-actions>
          <v-btn block color="primary" @click="deactivate">OK</v-btn>
        </v-card-actions>
      </v-card>
    </v-overlay>
  </div>
</template>
<script>

export default {
  head() {
    return {
      title: "QRCode",
      script: [
        {
          hid: "QRCode",
          src:
            "https://jojotoo-static.oss-cn-shanghai.aliyuncs.com/resources/script/qrcode.min.js",
          defer: true,
          callback: () => {
            this.QRCodeModuleLoaded = true;
          },
        },
      ],
    };
  },
  mounted() {
    const user = JSON.parse(localStorage.getItem("user"));
    this.name = user.name;
    this.accountId = user.id;
    this.balance = user.balance;
    this.fetchData();
  },
  data() {
    return {
      name: null,
      accountId: null,
      balance: null,
      profilePic: "https://picsum.photos/id/237/200/300",
      active: false,
      QRCodeModuleLoaded: false,
      url: null,
    };
  },
  methods: {
    fetchData() {
      this.url = this.accountId;
      this.$watch(
        (vm) => [vm.url, vm.url, vm.$refs],
        (val) => {
          const [url, QRCodeModuleLoaded, holder] = val;
          if (url && QRCodeModuleLoaded && holder.canvas) {
            this.getQRCode();
          }
        },
        {
          immediate: true,
          deep: true,
        }
      );
    },

    async getQRCode() {
      const options = {
        width: 400,
        height: 400,
        errorCorrectionLevel: "L",
        type: "image/png",
        rendererOpts: {
          quality: 1,
        },
      };
      /* global QRCode */
      await QRCode.toCanvas(this.$refs.canvas, this.url, options);
    },
    activate() {
      this.active = true;
    },
    deactivate() {
      this.active = false;
    }
  },
};
</script>
<style>
.canvas {
  width: 500px;
  height: 500px;
}
</style>