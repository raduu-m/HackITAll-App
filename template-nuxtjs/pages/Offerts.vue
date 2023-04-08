<template>
    <div>
        <v-container fluid>
            <v-row justify="center" align="center">
                <v-col v-for="(shop, index) in shops" :key="index" class="my-5">
                    <v-card @click="activate(index)" :elevation="activeIndex === index ? 16 : 4" class="mx-auto"
                        :max-width="400">
                        <v-img :src="shop.image" height="200px"></v-img>
                        <v-card-title class="text-h5">{{ shop.name }}</v-card-title>
                        <v-card-subtitle class="mb-3">{{ shop.offer.name }}</v-card-subtitle>
                        <v-card-text class="text-h6">
                            ${{ shop.offer.price }}
                        </v-card-text>
                    </v-card>
                </v-col>
            </v-row>
        </v-container>
        <v-overlay v-if="activeIndex >= 0" :value="true">
            <v-card class="mx-auto text-center" style="width: 400px; height: 400px;">
                <v-card-title class="text-h5">{{ shops[activeIndex].name }}</v-card-title>
                <v-card-subtitle class="mb-3">{{ selectedOffer.name }}</v-card-subtitle>
                <v-card-text class="text-h6">
                    ${{ shops[activeIndex].code }}
                </v-card-text>
                <v-card-text class="text-h6">
                    ${{ selectedOffer.price }}
                </v-card-text>
                <v-card-actions>
                    <v-btn block color="primary" @click="deactivate()">OK</v-btn>
                </v-card-actions>
            </v-card>
        </v-overlay>
    </div>
</template>
  
<script>
export default {
    data() {
        return {
            shops: [
                {
                    name: 'Footshop',
                    image:require('@/assets/item4.jpeg'),
                    offer: { name: 'Reducere folosind codul', price: "10%" },
                    code: "UXZ-321-BZT",
                },
                {
                    name: 'Emag',
                    image: require('@/assets/logo-emag.png'),
                    offer: { name: 'Reducere la electronice prin codul', price: "5%" },
                    code: "UXZ-321-BZT",
                },
                {
                    name: 'Quickmobile  ',
                    image: require('@/assets/quickmobile.jpg'),
                    offer: { name: 'Reducere folosind codul', price:"10%" },
                    code: "UXZ-321-BZT",
                }
            ],
            activeIndex: -1,
            selectedOffer: { name: '', price: 0 }
        };
    },
    methods: {
        activate(index) {
            this.activeIndex = index;
            this.selectedOffer = this.shops[index].offer;
            document.body.style.overflow = 'hidden';
        },
        deactivate() {
            this.activeIndex = -1;
            this.selectedOffer = { name: '', price: 0 };
            document.body.style.overflow = 'auto';
        }
    }
};
</script>
  