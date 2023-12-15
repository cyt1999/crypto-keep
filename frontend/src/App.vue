<template>
  <website-layout/>
</template>

<script>
import WebsiteLayout from "./components/WebsiteLayout.vue"
import { mapGetters } from 'vuex';
export default {
  name: 'App',
  components: {
    WebsiteLayout,
  },
  computed: {
    ...mapGetters(['contract'])
  },
  async created() {
    await this.$store.dispatch('initNear');
  },
};
</script>
