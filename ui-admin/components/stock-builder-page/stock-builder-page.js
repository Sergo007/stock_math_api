'use strict';


Vue.component('stock-builder-page', {
  template: `
  <div class="sg-builder">
    <div class="sg-controls">
      <span>(x,y)=({{itemPosition.x}},{{itemPosition.y}})</span> 
      <span>zoom: 
        <select v-model="zoom">
          <option value="10">10</option>
          <option value="20">20</option>
          <option value="30">30</option>
          <option value="40">40</option>
          <option value="50">50</option>
          <option value="60">60</option>
          <option value="70">70</option>
          <option value="80">80</option>
          <option value="90">90</option>
          <option value="100">100</option>
        </select>%
      </span>
      <select v-model="cell_type">
        <option value="">do nothing</option>
        <option value="edit_item_type">edit item type</option>
        <option value="empty">change to 'empty' type by click</option>
        <option value="empty_request_point">change to 'empty_request_point' type by click</option>
        <option value="stock_item">change to 'stock item' type by click</option>
        <option value="big_stock_item">change to 'big item' type by click</option>
        <option value="packaging_point">change to 'packaging point' type by click</option>
        <option value="wall">change to 'wall' type by click</option>
      </select>
      <button v-on:click="getJsonForRequest()">get json for request</button>
    </div>
    <div class="sg-controls-height"></div>
    <div class="sg-grid-container" :style="{zoom: zoom + '%'}">

      <div class="sg-row" v-for="(row, y) in stockGeometryItems">
        <div v-for="(item, x) in row" class="sg-item" 
          :class="{[item.type]: true, 'request_point_number': item.type=='empty' && item?.request_point_number!=null, 'selected': x==itemPosition.x && y==itemPosition.y}" 
          v-on:click="openEditCellPopup(item,x,y)">
          <span v-if="item.stock_item"><strong>{{item.stock_item.blockItemId}}</strong>/{{item.stock_item.rowId}}</span>
          <span v-if="item.type=='empty' && item?.request_point_number != null"><strong style="font-size: 35px;">{{item?.request_point_number}}</strong></span>
        </div>
      </div>
      
    </div>
    <edit-geometry-item-popup v-bind:submitButtonEvent="editCell"></edit-geometry-item-popup>
  </div>
  `,
  mounted() {
    this.stockGeometryItems = $storageService.getObject("stock-geometry", this.generatedEmpty(100))
  },
  computed: {
    // a computed getter

  },
  methods: {
    openRemovePractitionerPopup(practitioner){
      $popupService.getPopupComponent('confirmation-message:removing-practitioner').open(practitioner);
    },
    openEditCellPopup(item,x,y){
      // console.log("openEditCellPopup", x,y, JSON.stringify(item, null, 2))
      this.itemPosition = {x:x,y:y}
      if (this.cell_type == "") return
      if (this.cell_type == "edit_item_type"){
        $popupService.getPopupComponent('edit-geometry-item-popup').open(item,x,y);
        return;
      }
      if (this.cell_type == "empty_request_point") {
        this.editCell({
          type: "empty",
          request_point_number: 0
        },x,y)
      } else {
        this.editCell({
            type: this.cell_type
        },x,y)
      }
      
    },
    getOptiumRoute(request) {
      // return axios.post('https://stock-math-api-5bjvbywvdq-uc.a.run.app/calculate_optimal_path', request)
      return axios.post('/calculate_optimal_path', request)
      .then((response) => {
        if (!response?.data?.algorithm) {
          console.log("getOptiumRoute", response.data)
        }
        return response?.data
      })
      .catch((error) => {
        console.log("getOptiumRoute", error)
        return Promise.reject(error)
      })
    },
    getDIstanceMatrixText(request) {
      return axios.post('/calc_distances_matrix_text', request)
      .then((response) => {
        return response?.data
      })
      .catch((error) => {
        console.log("getOptiumRoute", error)
        return Promise.reject(error)
      })
    },
    editCell(item,x,y){
      // console.log("editCell", x,y, JSON.stringify(item, null, 2))
      Vue.set(this.stockGeometryItems[y], x, item)
      $storageService.setObject("stock-geometry", this.stockGeometryItems)
    },
    generatedEmpty(size) {
      let emptyItem = {
        type: "empty"
      }
      let arr = []
      for(let i = 0;i<size;i++) {
        let row = []
        for(let j = 0;j<size;j++) {
          row.push(emptyItem)
        }
        arr.push(row)
      }
      return arr
    },
    getJsonForRequest() {
      let stockGeometryItems = this.stockGeometryItems;
      let text = ''
      let request = {
        geometry: [],
        points: []
      }
      let arr = []
      let points = []
      let pointsCount = 1;
      let start = []
      for(let i = 0;i<stockGeometryItems.length;i++) {
        let row = []
        for(let j = 0;j<stockGeometryItems[i].length;j++) {
          let item = '_';
          if (stockGeometryItems[i][j].type != "empty") {
            item = 'W'
          }
          if (stockGeometryItems[i][j].type == "packaging_point") {
            item = 'S'
            start = [i, j]
          }
          row.push(item)
          if (stockGeometryItems[i][j].type == "empty" && stockGeometryItems[i][j]?.request_point_number!=null) {
            item = 'P'
            points.push([i, j]);
            stockGeometryItems[i][j].request_point_number = 0;
            pointsCount++;
          }
          text = text + item
        }
        text = text + '\n'
        arr.push(row)
      }
      request.geometry = arr
      request.points = [start, ...points]
      // request.points = [
      //   [
      //     67,
      //     45
      //   ],
      //   [
      //     32,
      //     19
      //   ],
      //   [
      //     2,
      //     8
      //   ],
      //   [
      //     8,
      //     40
      //   ],
      //   [
      //     26,
      //     10
      //   ],
      //   [
      //     14,
      //     44
      //   ],
      //   [
      //     2,
      //     35
      //   ],
      //   [
      //     29,
      //     19
      //   ]
      // ]
      console.log("points count: ",request.points.length)
      console.log("points: ", JSON.stringify(request.points, null, 2));
      // console.log(text)
      // console.log(JSON.stringify(request))
      this.getOptiumRoute(request).then(resp=>{
        console.log("optium route distance",resp.distance);
        console.log("distance_matrix_time: ",resp.distance_matrix_time);
        console.log("traveling_salesman_time: ",resp.traveling_salesman_time);
        console.log("optium route",resp.path);
        resp.path.forEach(([i,j], index) => {
          stockGeometryItems[i][j].request_point_number = index;
        });
      })
      this.getDIstanceMatrixText(request).then(resp=>{
        console.log("distance matrix: ");
        console.log(resp);
      })
    }
  },
  data: function () {
    return {
      isShowLogs: false,
      itemPosition: {x:0, y:0},
      zoom: "30",
      cell_type: "",
      stockGeometryItems: []
    }
  },
});
