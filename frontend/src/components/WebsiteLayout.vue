<template>
  <a-layout v-if="currentUser">
    
    <a-layout-header :style="{ position: 'fixed', zIndex: 1, width: '100%', paddingInline:'10px',display:'flex' , alignItems:'center',justifyContent: 'space-between',}">
      
        <h1 :style="{color:'#FFF',marginTop:'5px'}">CryptoKeep</h1>
  
        <a-input v-model:value="searchInput" @change="search"  placeholder="搜索密码" :style="{width:'50%',marginLeft:'15%'}">
          <template #prefix>
            <SearchOutlined />
          </template>
        </a-input>
  
        <div :style="{display:'flex',alignItems:'center'}">
          <h3 style="color:#FFF">{{currentUser.accountId}} {{currentUser.balanceInNear}} near</h3>
          <a-button :style="{marginLeft: '10px', backgroundColor: '#1EE3CF'}" type="primary" @click="signOut()">Logout</a-button>
        </div>

    </a-layout-header>

    <a-layout>
        <a-layout-sider :style="{ height: '100vh', position: 'fixed'}">
          <a-menu mode="inline" :selectedKeys="selectedKeys"  :style="{ height: '100%', borderRight: 0, paddingTop: '64px' }">
            <a-menu-item key="1" >
              <KeyOutlined />
              <span>密码</span>
            </a-menu-item>
            <a-menu-item key="2">
              <CheckCircleOutlined />
              <span>检查</span>
            </a-menu-item>
            <a-menu-item key="3">
              <SettingOutlined />
              <span>设置</span>
            </a-menu-item>
          </a-menu>
        </a-layout-sider>

      <a-layout-content :style="{ marginTop: '64px', backgroundColor: '#FFF'  }">
        <div :style="{width: '50%',margin:'auto'}">
          
          <div :style="{marginTop:'10px',display:'flex'}">
            <h1 style="font-size:1.50em"> 密码 </h1>
            <a-button  @click="showModal" :style="{borderColor:'#AECBFA',borderRadius:'20px',color:'#1E64D4',marginLeft:'auto'}">添加</a-button>
            <!-- <add-password-form style="margin-left: auto"></add-password-form> -->
          </div>

          <p>
            您可创建、保存和管理您的密码，以便轻松登录网站和应用。
          </p>
          <!-- 展示列表 -->
          <a-card v-show="state" :style="{boxShadow:'2px 2px 5px rgba(0, 0, 0, 0.3)'}">
            <a-list  :data-source="data">
              <template #renderItem="{ item }">
                <a-list-item  class="clickable-div"  @click="showModal(item,'edit')">
                  {{ item.website }}
                  <a-button @click.stop="deletePasswords(item.index)" shape="circle" :style="{float:'right',color:'#FF5759',borderColor:'#FFABAB'}">
                    <DeleteOutlined />
                  </a-button>
                </a-list-item>
              </template>
            </a-list>
          </a-card>
          
          <!-- 结果列表 -->
          <a-card v-show="IsResultDisplayed" :style="{boxShadow:'2px 2px 5px rgba(0, 0, 0, 0.3)'}">
            <a-list :data-source="searchResult">
              <template #renderItem="{ item }">
                <a-list-item  class="clickable-div" @click="showModal(item,'edit')">
                  {{ item.website }}
                  <a-button @click.stop="deletePasswords(item.index)" shape="circle" :style="{float:'right',color:'#FF5759',borderColor:'#FFABAB'}">
                    <DeleteOutlined />
                  </a-button>
                </a-list-item>
              </template>
            </a-list>
          </a-card>

        </div>
    
        
      </a-layout-content>
      
    </a-layout>
  </a-layout>
  

  <!-- 登录页面 -->
  <div v-else :style="{ display: 'flex', flexDirection: 'column', justifyContent: 'center',alignItems: 'center', height: '100vh',  backgroundColor: '#e8e8e8'}">
          <img class="logo" alt="cryptokeep logo" src="../assets/cryptokeep.png">
          <h1 :style="{color:'1F1F1F'}"> 欢迎使用 Crypto Keep ! </h1>
          <a-button style="background-color: #000000;" type="primary" size="large" @click="signIn()">Login</a-button>
  </div>

  <!-- 密码详情页  -->
  <a-modal :open="openModel" :model="model" :title="modelTitle" @ok="handleOk" @cancel="handleCancel" cancelText="取消" okText="确定">
      <a-form :model="model" layout="vertical" autocomplete="off">
        <a-form-item label="网站" name="website" >
          <a-input v-model:value="model.website" />
        </a-form-item>

        <a-form-item label="用户名" name="username">
          <a-input v-model:value="model.username" />
        </a-form-item>

        <a-form-item label="密码" name="password">
          <a-input-password v-model:value="model.password" />
          <span :style="{color:'#787e84',fontSize:'12px' }">
            请确保您保存的是您目前用于此网站的密码
          </span>
        </a-form-item>

        <a-form-item label="备注" name="note" >
          <a-input v-model:value="model.note" />
        </a-form-item>
      </a-form>
  </a-modal>

</template>

<script>
import { mapGetters } from 'vuex';
import { create, open } from '@nearfoundation/near-js-encryption-box';
import {SearchOutlined,KeyOutlined,CheckCircleOutlined,SettingOutlined,DeleteOutlined} from '@ant-design/icons-vue'; 
// import {message} from 'ant-design-vue';

export default {
  name: 'CryptoKeep',
  components: {
    SearchOutlined,
    KeyOutlined,
    CheckCircleOutlined,
    SettingOutlined,
    DeleteOutlined
  },
  computed: {
    ...mapGetters(['currentUser', 'contract', 'wallet', 'nearConfig'])
  },
  data() {
    return {
      state: true,
      IsResultDisplayed: false,
      searchInput:'',
      modelTitle:'',
      modelMethod:'',
      publicKey: '4DXgxXFRVh9HBegoj2RpdzJ6sbYiW1HxWixRFwkheweH',
      privateKey: '5YsDBVUrQ8fCf1AnQNTWaSSxhGdvsGwRJc6SSTfsEumpo3DAzrJfp1NCotNhU7yGpw8D6hruugyxdzmDhT3C9tpR',
      openModel: false,
      selectedKeys : ['1'],
      data: [],
      model:{},
      searchResult: [],
    }
  },

  async created() {
    console.log("created")
    // 处理公私钥
    // if (localStorage.getItem('undefined_wallet_auth_key')) {
    //   const wallet_auth_key = JSON.parse(localStorage.getItem('undefined_wallet_auth_key'));
    //   this.publicKey = wallet_auth_key.allKeys[0];
    //   this.privateKey = localStorage.getItem(`near-api-js:keystore:${wallet_auth_key.accountId}:testnet`);
    // }
    // 初始化Near，每次刷新初始化，不然无法调用合约方法。
    await this.$store.dispatch('initNear');
    this.getPasswords();
  },


  methods: {
    // Sign in
    signIn() {
      this.wallet.requestSignIn(
        this.nearConfig.contractName,
        'Crypto Keep'
      );
    },

    signOut() {
      this.wallet.signOut();
      window.location.replace(window.location.origin + window.location.pathname)
    },

    // 获取密码列表-从local Storage
    // 获取密码列表-从合约
    async getPasswords() {
      // 清空密码列表
      this.data = [];
      try {
        const response = await this.contract.get_passwords(
          {
            account_id: this.currentUser.accountId,
          },
          // this.gas,
          // Big(this.gamble).times(10**24).toFixed()
        );
        
        for (let i = 0; i < response.length; i++) {
          const index = response[i][0]
          // console.log('解密数据',this.dateDecrypting(response[i][1]));
          const passwordInfo = JSON.parse(this.dateDecrypting(response[i][1]));
          // 如果数据解密不成功，将会返回none，那么这里将会报错。
          passwordInfo.index = index;
          this.data.push(passwordInfo);
          // 调用删除接口，清空数据
          // this.deletePasswords(index);
        }

      } catch (e) {
        console.log(e);
      }
    },

    // 添加密码
    async addPassword(secret) {
      try {
        const response = await this.contract.add_password(
          {
            account_id: this.currentUser.accountId,
            encrypted_password_info: secret
          }
        );
        // <a-alert message="add successful" type="success" />
        console.log('add password successful', response);
      } catch (e) {
        // <a-alert message="add failed" type="error" />
        console.error('add password  failed', e);
      }

      // 重新查询，更新数据
      this.getPasswords();
    },

    // 更新密码
    async updatePasswords(index, new_password_info) {
      try {
        const response = await this.contract.update_password(
          {
            account_id: this.currentUser.accountId,
            index: index,
            new_encrypted_password_info: new_password_info
          }
        );

        console.log('update password successful',response);

      } catch (e) {
        console.log('update password failed',e);
      }
      // 重新查询，更新数据
      this.getPasswords();
    },

    // 删除密码
    async deletePasswords(index) {
      // message.error("删除成功",index)
      try {
        const response = await this.contract.delete_password(
          {
            account_id: this.currentUser.accountId,
            index: index
          }
        );

        console.log(index,"密码删除成功",response);

      } catch (e) {
        console.log(e);
      }
      
      // 重新查询，更新数据
      this.getPasswords();
    },

    // 数据加密
    dateEncrypting(message) {
      const { secret, } = create(message, this.publicKey, this.privateKey, "nozjlJALKKoGoAVI+voQdDR1ud0mYsXS");
      return secret;

    },

    // 数据解密
     dateDecrypting(message) {
      const messageReceived = open(message, this.publicKey, this.privateKey, "nozjlJALKKoGoAVI+voQdDR1ud0mYsXS");
      return messageReceived;
     },

    showModal(item,method) {
      this.openModel = true;
      if (method == 'edit') {
        this.modelTitle = '编辑密码';
        // 拷贝itme，默认是引用
        this.model =  JSON.parse(JSON.stringify(item));
        // 当从编辑页面点击确定的时候，触发update操作。
        this.modelMethod = 'update'
      }else {
        this.modelTitle = '新增密码';
        // 当从新增页面点击确定的时候，触发create操作。
        this.modelMethod = 'create';
      }
    },

    handleOk() {

      if (this.modelMethod == 'update' ) {
        const index = this.model.index;
        // 删除用来标识数据的index
        delete this.model.index;
        // 数据加密
        const secret = this.dateEncrypting(JSON.stringify(this.model));
        // 更新密码
        this.updatePasswords(index,secret);
      }else {
        // 数据加密
        const secret = this.dateEncrypting(JSON.stringify(this.model));
        // 新增密码 
        this.addPassword(secret);
      }

      // 重置model
      this.model = {};
      
      this.openModel = false;
    },

    handleCancel() {
      // 重置model
      this.model = {};
      this.openModel = false;
    },

    // 搜索功能
    search() {
        this.searchResult = [];

        //循环模拟数据的数组
        this.data.map((msg) => {
          //拿当前json的id、name、time去分别跟输入的值进行比较
          //indexOf 如果在检索的字符串中没有出现要找的值是会返回-1的，所以我们这里不等于-1就是假设输入框的值在当前json里面找到的情况
          if (msg.website.indexOf(this.searchInput) != -1) {
            //然后把当前json添加到list数组中
            this.searchResult.push(msg);
          }
        })

        // 判断searchInput展示列表，如果输入了就展示没输入就不展示
        if (this.searchInput.length > 0) {
          this.state = false;
          this.IsResultDisplayed = true;
        }else{
          this.state = true;
          this.IsResultDisplayed = false;
        }
      },
  }
};
</script>
<style scoped>
.logo {
  width: 40%;
  height: 40%;
  margin: 10px 0 10px 0;
  /* padding: 10px 10px 10px 10px; */
}
.ant-layout-sider-children {
  background: white;
}

/* 侧边栏宽度 */
.ant-layout-sider-dark {
      flex: 0 0 250px !important;
      max-width: 250px !important;
      min-width: 250px !important;
      width: 250px !important;
    }

.clickable-div {
  cursor: pointer; /* 鼠标悬停时显示手型光标 */
}

.clickable-div:hover {
  background-color: #dcdcdc; /* 鼠标悬停时改变背景色 */
}
</style>
