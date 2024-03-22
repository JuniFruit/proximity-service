<script lang="ts">
import {Input} from '@/components/ui/input';
import {Button} from '@/components/ui/button';
import {Crosshair1} from 'svelte-radix';
</script>

<div class="w-full flex justify-center">
  <div class="inner_container">
    <Input placeholder="Filter" class="bg-white h-7" />
    <Button variant="outline" class="w-12" size="icon">
      <Crosshair1 />
    </Button>
  </div>
</div>


<style scoped>
.inner_container {
  @apply px-2 mx-auto space-x-2 flex items-center gap-5;

}

</style>
