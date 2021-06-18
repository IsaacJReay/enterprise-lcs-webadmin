import React from "react";
import { Modal, Input, Form, Button } from "antd";
import { FiX } from "react-icons/fi";

const DNSRename = ({ visible, handleCancel, handleOk }) => {
  return (
    <React.Fragment>
      <Modal
        title="Rename domain name"
        visible={visible}
        onCancel={handleCancel}
        onOk={handleOk}
        closeIcon={<FiX className="close-icon" />}
        footer={null}
      >
        <Form>
          <Form.Item>
            <Input placeholder="Text here ..." size="large" />
          </Form.Item>
          <Form.Item>
            <Button type="primary" htmlType="submit" className="submit-button">
              Submit
            </Button>
          </Form.Item>
        </Form>
      </Modal>
    </React.Fragment>
  );
};

export default DNSRename;
