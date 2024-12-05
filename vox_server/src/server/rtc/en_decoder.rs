use anyhow::Result;
use async_trait::async_trait;
use opus::Decoder as OpusDecoder;
use std::sync::Arc;

// 音频解码器trait
#[async_trait]
pub trait AudioDecoder: Send + 'static {
    // 解码音频数据
    async fn decode(&mut self, input: &[u8]) -> Result<Vec<i16>>;
    // 获取采样率
    fn sample_rate(&self) -> u32;
    // 获取通道数
    fn channels(&self) -> u16;
}

// Opus解码器实现
pub struct OpusAudioDecoder {
    decoder: OpusDecoder,
    sample_rate: u32,
    channels: u16,
}

impl OpusAudioDecoder {
    pub fn new(sample_rate: u32, channels: u16) -> Result<Self> {
        Ok(Self {
            decoder: OpusDecoder::new(sample_rate, opus::Channels::Mono)?,
            sample_rate,
            channels,
        })
    }
}

#[async_trait]
impl AudioDecoder for OpusAudioDecoder {
    async fn decode(&mut self, input: &[u8]) -> Result<Vec<i16>> {
        let mut output = vec![0i16; 960 * self.channels as usize]; // 20ms at 48kHz
        let samples = self.decoder.decode(input, &mut output, false)?;
        output.truncate(samples * self.channels as usize);
        Ok(output)
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn channels(&self) -> u16 {
        self.channels
    }
}

// 工厂函数用于创建不同类型的解码器
#[derive(Debug)]
pub enum DecoderType {
    Opus,
    // 未来可以添加更多解码器类型
    // AAC,
    // MP3,
    // etc.
}

pub fn create_decoder(
    decoder_type: DecoderType,
    sample_rate: u32,
    channels: u16,
) -> Result<Box<dyn AudioDecoder>> {
    match decoder_type {
        DecoderType::Opus => {
            let decoder = OpusAudioDecoder::new(sample_rate, channels)?;
            Ok(Box::new(decoder))
        }
        // 未来添加更多解码器类型的匹配
    }
}

// 音频编码器trait，为未来可能的编码功能预留
#[async_trait]
pub trait AudioEncoder: Send + Sync {
    async fn encode(&mut self, input: &[i16]) -> Result<Vec<u8>>;
    fn sample_rate(&self) -> u32;
    fn channels(&self) -> u16;
}

// 统一的音频处理器结构体
pub struct VoxDecoder {
    decoder: Box<dyn AudioDecoder>,
}

impl VoxDecoder {
    pub fn new(decoder_type: DecoderType, sample_rate: u32, channels: u16) -> Result<Self> {
        let decoder = create_decoder(decoder_type, sample_rate, channels)?;
        Ok(Self { decoder })
    }

    pub async fn decode(&mut self, input: &[u8]) -> Result<Vec<i16>> {
        self.decoder.decode(input).await
    }

    pub fn sample_rate(&self) -> u32 {
        self.decoder.sample_rate()
    }

    pub fn channels(&self) -> u16 {
        self.decoder.channels()
    }
}