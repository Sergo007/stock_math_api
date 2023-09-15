// Шаги алгоритма ECMA-262, 5-е издание, 15.4.4.14
// Ссылка (en): http://es5.github.io/#x15.4.4.14
// Ссылка (ru): http://es5.javascript.ru/x15.4.html#x15.4.4.14
// funcEqual: for object searching 
// Example: 
Array.prototype.objectIndexOf = function(searchElement, fromIndex, funcEqual) {
  var k, functionEqual;
  functionEqual = funcEqual
  if (functionEqual == null || functionEqual == undefined) {
    functionEqual = function(element, searchElement) { return element === searchElement }
  }
  if (this == null) {
    throw new TypeError('"this" is null or not defined');
  }
  var O = Object(this);
  var len = O.length >>> 0;
  if (len === 0) {
    return -1;
  }
  var n = +fromIndex || 0;
  if (Math.abs(n) === Infinity) {
    n = 0;
  }
  if (n >= len) {
    return -1;
  }
  k = Math.max(n >= 0 ? n : len - Math.abs(n), 0);
  while (k < len) {
    if (k in O && functionEqual(O[k], searchElement)) {
      return k;
    }
    k++;
  }
  return -1;
};



if (!Object.assign) {
  Object.defineProperty(Object, 'assign', {
    enumerable: false,
    configurable: true,
    writable: true,
    value: function(target, firstSource) {
      'use strict';
      if (target === undefined || target === null) {
        throw new TypeError('Cannot convert first argument to object');
      }

      var to = Object(target);
      for (var i = 1; i < arguments.length; i++) {
        var nextSource = arguments[i];
        if (nextSource === undefined || nextSource === null) {
          continue;
        }

        var keysArray = Object.keys(Object(nextSource));
        for (var nextIndex = 0, len = keysArray.length; nextIndex < len; nextIndex++) {
          var nextKey = keysArray[nextIndex];
          var desc = Object.getOwnPropertyDescriptor(nextSource, nextKey);
          if (desc !== undefined && desc.enumerable) {
            to[nextKey] = nextSource[nextKey];
          }
        }
      }
      return to;
    }
  });
}