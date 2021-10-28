
import tradesResponseText from './trades.js';

const sleep = (time) => new Promise(resolve => setTimeout(resolve, time));

(async function() {
    await sleep(1000);
    
    const getKey = (item) => {
        return [
            item.appid,
            item.classid,
            item.instanceid
        ].join('_');
    };
    console.time('parse');
    const parsed = JSON.parse(tradesResponseText);
    const descriptions = parsed.descriptions
        .reduce((descriptions, classinfo) => {
            descriptions[getKey(classinfo)] = classinfo;
            
            return descriptions;
        });
    const getItemsFromTrade = (items) => {
        return (items || [])
            .map((item) => {
                const key = getKey(item);
                const description = descriptions[key];
                
                if (!description) {
                    return null;
                }
                
                return description.market_hash_name;
            })
            .join(', ');
    };
    const html = parsed.trades
        .map((trade) => {
            return [
                trade.tradeid,
                getItemsFromTrade(trade.assets_given),
                getItemsFromTrade(trade.assets_received)
            ].join('\n');
        })
        .join('\n\n');
    console.timeEnd('parse');
}());