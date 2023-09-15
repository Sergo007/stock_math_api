/*
example for use table material
<Table>
  <TableHead>
    <TableRow inHead>
      <TableCell inHead >Key</TableCell>
      <TableCell inHead align="center">Type</TableCell>
    </TableRow>
  </TableHead>
  <TableBody>
    <TableRow>
      <TableCell scope="row">
        value1
      </TableCell>
      <TableCell align="center">string</TableCell>
    </TableRow>
    <TableRow>
      <TableCell scope="row">
        value2
      </TableCell>
      <TableCell align="center">int</TableCell>
    </TableRow>
    <TableRow>
      <TableCell scope="row">
        value3
      </TableCell>
      <TableCell align="center">char(50)</TableCell>
    </TableRow>
  </TableBody>
</Table>

 */


Vue.component('Table', {
  template: `
<table class="MuiTable-root">
<slot></slot>
</table>
  `
});

Vue.component('TableHead', {
  template: `
<thead class="MuiTableHead-root">
<slot></slot>
</thead>
  `
});

Vue.component('TableBody', {
  template: `
<tbody class="MuiTableBody-root">
<slot></slot>
</tbody>
  `
});

Vue.component('TableRow', {
  template: `
<tr v-bind:class="['MuiTableRow-root', transformInHead]">
<slot></slot>
</tr>
  `,
  props: {
    inHead: {
      type: Boolean,
      default: false,
    }
  },
  computed: {
    transformInHead: function () {
      if(this.inHead) {
        return 'MuiTableRow-head';
      }
      return '';
    }
  }
});

Vue.component('TableCell', {
  template: `
<th v-bind:class="['MuiTableCell-root', transformInHead, alignPosition]">
<slot></slot>
</th>
  `,
  props: {
    inHead: {
      type: Boolean,
      default: false,
    },
    align: {
      type: String,
      default: "left",
      validator: function (value) {
        // Значение должно соответствовать одной из этих строк
        return ['right', 'center', 'left', 'justify'].indexOf(value) !== -1
      }
    },
  },
  computed: {
    alignPosition: function () {
      if(this.align === 'right') {
        return 'MuiTableCell-alignRight';
      }
      if(this.align === 'center') {
        return 'MuiTableCell-alignCenter';
      }
      if(this.align === 'left') {
        return 'MuiTableCell-alignLeft';
      }
      if(this.align === 'justify') {
        return 'MuiTableCell-alignJustify';
      }
      return '';
    },
    transformInHead: function () {
      if(this.inHead) {
        return 'MuiTableCell-head';
      }
      return 'MuiTableCell-body';
    }
  }
});
