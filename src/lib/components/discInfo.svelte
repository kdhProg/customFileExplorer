
<script>
    import { getDriveInfo } from "$lib/api";
    import { drives } from "$lib/store";

    let drives_infos = [];


    async function get_drive_info() {
        try {
            drives_infos = await getDriveInfo();
        } catch (error) {
            console.error('Failed to fetch drive list:', error);
        }
    }

    get_drive_info();
</script>
<div>
    <br/><br/>
    {#each drives_infos as disc}
      <div class="disc-case-wrapper">
        <div>
            <div>
                <img class="disc-icon-img" src="/icons/disc.png" alt="">
            </div>
            <div class="disc-name">
                {(disc.mount_point).replace(/([A-Z]):\\/, '$1 drive')}
            </div>
        </div>
        <div class="graph-capacity-wrapper">
            <div class="graph-wrapper">
                <div class="used-space"
                    style="width : {Math.floor((disc.total_space-disc.available_space)/Math.floor(disc.total_space)*100)}%">
                    <!-- {Math.floor((disc.total_space-disc.available_space))}GB -->
                </div>

                <div class="remainder-space" 
                    style="width : {Math.floor(Math.floor(disc.available_space)/Math.floor(disc.total_space)*100)}%">
                    <!-- {Math.floor(disc.available_space)}GB -->
                </div>
            </div>
            <div>
                {Math.floor((disc.total_space-disc.available_space))}GB / {Math.floor(disc.total_space)}GB
            </div>
        </div>
      </div>
      <br/><br/>
    {/each}
</div>
<style>
    .disc-case-wrapper{
        display: flex;
        flex-direction: row;

        width: 300px;
        height: 80px;

        /* border: 1px solid red; */
    }

    .disc-name{
        text-align: center;
    }

    .disc-icon-img{
        width: 40px;
        height: 25px;
    }

    .graph-capacity-wrapper{
        display: flex;
        flex-direction: column;

        margin-left: 10px;
    }

    .graph-wrapper{
        display: flex;
        flex-direction: row;

        width: 200px;

        margin-top: 15px;

        border: 0.5px solid black;
        /* border-radius: 5%; */
    }

    .remainder-space{
        background-color: rgb(247, 254, 255);
        height: 10px;

        /* margin-top: 20px; */

    }

    .used-space{
        background-color: rgb(130, 153, 244);
        height: 10px;

        /* margin-top: 20px; */
    }
</style>