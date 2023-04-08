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
                    name: 'Shop A',
                    image: 'https://picsum.photos/400/300?random=1',
                    offer: { name: 'Offer 1', price: 10 },
                    code: "UXZ-321-BZT",
                },
                {
                    name: 'Shop B',
                    image: 'https://picsum.photos/400/300?random=2',
                    offer: { name: 'Offer 4', price: 40 },
                    code: "UXZ-321-BZT",
                },
                {
                    name: 'Shop C',
                    image: 'https://picsum.photos/400/300?random=3',
                    offer: { name: 'Offer 7', price: 70 },
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
  