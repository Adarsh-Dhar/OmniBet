const { renderHook, act } = require('@testing-library/react-hooks');
const { useStore, useTokenStore, usePriceStore } = require('@/states/state');

describe('Zustand Store', () => {
  describe('useStore', () => {
    test('should update tokenSelected', () => {
      const { result } = renderHook(() => useStore());
      act(() => {
        result.current.changeToken('newToken');
      });
      expect(result.current.tokenSelected).toBe('newToken');
    });

    test('should update priceCalaulated', () => {
      const { result } = renderHook(() => useStore());
      act(() => {
        result.current.changePrice(100);
      });
      expect(result.current.priceCalaulated).toBe(100);
    });

    test('should update address', () => {
      const { result } = renderHook(() => useStore());
      act(() => {
        result.current.changeAddress('0x1234567890abcdef');
      });
      expect(result.current.address).toBe('0x1234567890abcdef');
    });
  });

  describe('useTokenStore', () => {
    test('should update primaryToken', () => {
      const { result } = renderHook(() => useTokenStore());
      act(() => {
        result.current.changePrimaryToken('BTC');
      });
      expect(result.current.primaryToken).toBe('BTC');
    });

    test('should update referenceToken', () => {
      const { result } = renderHook(() => useTokenStore());
      act(() => {
        result.current.changeReferenceToken('ETH');
      });
      expect(result.current.referenceToken).toBe('ETH');
    });
  });

  describe('usePriceStore', () => {
    test('should add a new price', () => {
      const { result } = renderHook(() => usePriceStore());
      act(() => {
        result.current.addPrice('100');
      });
      expect(result.current.prices).toEqual(['100']);
    });

    test('should remove the last price', () => {
      const { result } = renderHook(() => usePriceStore());
      act(() => {
        result.current.addPrice('100');
        result.current.addPrice('200');
        result.current.removePrice();
      });
      expect(result.current.prices).toEqual(['100']);
    });

    test('should add a new date', () => {
      const { result } = renderHook(() => usePriceStore());
      act(() => {
        result.current.addDates('2023-04-28');
      });
      expect(result.current.dates).toEqual(['2023-04-28']);
    });

    test('should remove the last date', () => {
      const { result } = renderHook(() => usePriceStore());
      act(() => {
        result.current.addDates('2023-04-28');
        result.current.addDates('2023-04-29');
        result.current.removeDate();
      });
      expect(result.current.dates).toEqual(['2023-04-28']);
    });
  });
});